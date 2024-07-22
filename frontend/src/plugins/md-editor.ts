import VueMarkdownEditor from '@kangc/v-md-editor';
import '@kangc/v-md-editor/lib/style/base-editor.css';
import vuepressTheme from '@kangc/v-md-editor/lib/theme/vuepress.js';
import '@kangc/v-md-editor/lib/theme/style/vuepress.css';
import VMdPreview from '@kangc/v-md-editor/lib/preview';
import '@kangc/v-md-editor/lib/style/preview.css';
import Prism from 'prismjs';

VueMarkdownEditor.use(vuepressTheme, {
  Prism,
});

VMdPreview.use(
  vuepressTheme, {
    Prism,
  }
)
export {VueMarkdownEditor, VMdPreview};