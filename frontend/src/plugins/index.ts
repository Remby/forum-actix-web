import { App } from "vue";
import {VueMarkdownEditor,VMdPreview} from "./md-editor";
const pluginList = [
    VueMarkdownEditor,
    VMdPreview,
]

const plugins = {
    install(app: App<Element>) {
        Object.
            entries(pluginList)
            .forEach(([, v]) => {
                app.use(v)
            })
    }
}

export default plugins;