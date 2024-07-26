use super::chat_server::{
    ChatServer, Connect, Disconnect, GroupMessage, JoinRoom, ListRooms, Message, PrivateMessage,
};
use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsChatSession {
    pub id: usize,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<ChatServer>,
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("WebSocket Client heartbeat failed, disconnecting!");

                act.addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
                username: self.name.clone().unwrap_or_default(),
            })
            .into_actor(self)
            .then(|res, act, _ctx| {
                match res {
                    Ok(id) => act.id = id,
                    _ => _ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("stream handler");
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("{:#?}", text);
                let text = text.trim();
                if text.starts_with('/') {
                    let parts: Vec<&str> = text.splitn(2, ' ').collect();
                    match parts[0] {
                        "/list" => {
                            self.addr
                                .send(ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => ctx.text("Failed to list rooms"),
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx);
                        }
                        "/join" => {
                            if parts.len() == 2 {
                                self.room = parts[1].to_string();
                                self.addr.do_send(JoinRoom {
                                    id: self.id,
                                    room: self.room.clone(),
                                });
                                ctx.text("Joined room");
                            } else {
                                ctx.text("Room name is required");
                            }
                        }
                        "/pm" => {
                            if parts.len() == 2 {
                                let pm_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();
                                if pm_parts.len() == 2 {
                                    self.addr.do_send(PrivateMessage {
                                        from: self.name.clone().unwrap_or_default(),
                                        to: pm_parts[0].to_string(),
                                        content: pm_parts[1].to_string(),
                                    });
                                } else {
                                    println!("Usage: /pm <user> <message>");
                                    ctx.text("Usage: /pm <user> <message>");
                                }
                            }
                        }
                        _ => ctx.text("Unknown command"),
                    }
                } else {
                    let msg = if let Some(ref name) = self.name {
                        format!("{name}: {text}")
                    } else {
                        text.to_string()
                    };
                    self.addr.do_send(GroupMessage {
                        id: self.id,
                        room: self.room.clone(),
                        content: msg,
                    });
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                
                println!("ping ");
                println!("{:#?}", msg);
                
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
                println!("pong");
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
