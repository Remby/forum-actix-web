use actix::prelude::*;

use rand::{rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// 服务器发送给会话的消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// 用户连接时的消息
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub username: String,
}

/// 用户断开连接时的消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// 发送到特定房间的消息
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct GroupMessage {
    pub id: usize,
    pub room: String,
    pub content: String,
}

/// 发送私人消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct PrivateMessage {
    pub from: String,
    pub to: String,
    pub content: String,
}

/// 请求房间列表的消息
#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

/// 加入房间的消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub id: usize,
    pub room: String,
}

/// ChatServer 管理所有聊天室和用户会话
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            visitor_count,
        }
    }

    fn send_group_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
    fn send_private_message(&self, to: &str, from: &str, message: &str) {
        // Find the recipient's id based on the username 'to'
        let recipient_id = self.sessions.iter().find_map(|(&id, addr)| {
            // Assuming you have a way to get the user's name from the address, implement that logic here.
            // Example: if get_username_by_id(id) == to { Some(id) } else { None }
            // However, the `Recipient` type does not directly support extracting the username,
            // so you need to manage this mapping separately.
            if
            /* your logic to match username with addr */
            false {
                Some(id)
            } else {
                None
            }
        });

        if let Some(id) = recipient_id {
            if let Some(addr) = self.sessions.get(&id) {
                let _ = addr.do_send(Message(format!(
                    "Private message from {}: {}",
                    from, message
                )));
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        self.rooms.entry("main".to_owned()).or_default().insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_group_message("main", &format!("Total visitors {count}"), 0);

        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            for (_, sessions) in &mut self.rooms {
                sessions.remove(&msg.id);
            }
        }
    }
}

impl Handler<GroupMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: GroupMessage, _: &mut Context<Self>) {
        self.send_group_message(&msg.room, &msg.content, msg.id);
    }
}

impl Handler<PrivateMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessage, _: &mut Context<Self>) {
        self.send_private_message(&msg.to, &msg.from, &msg.content);
    }
}

impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let rooms = self.rooms.keys().cloned().collect();
        MessageResult(rooms)
    }
}

impl Handler<JoinRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
        let JoinRoom { id, room } = msg;

        for (_, sessions) in &mut self.rooms {
            sessions.remove(&id);
        }
        self.rooms.entry(room.clone()).or_default().insert(id);
        self.send_group_message(&room, "Someone joined the room", id);
    }
}
