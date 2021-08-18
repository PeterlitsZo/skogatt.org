import React from 'react';

import {
  app,
  header as _header,
  logo,
  cursor,
  footnote,
  line,
  content as _content
} from './App.module.scss';

import {LikeButton, DislikeButton} from './Button';
import {CommentHead, Comment} from './Comment';
import {ButtonsGroup, Placeholder} from './ButtonsGroup';

function App() {
  let header = (
    <header className={_header}>
      <div className={logo}>
        <p>Peterlits</p><p className={cursor} />
      </div>
      <div className={footnote}>
        <p>生活 · 技术 · 存在 · 我</p><p className={line} /><p>●</p>
      </div>
    </header>
  );

  let content = (
    <div className={_content}>
      <p>Peterlits 其人，本名周泓余，现于苏州大学计算机科学与技术系在读。</p>
      <p>这里是 Peterlits 的小网站，它基于 nignx 反向代理，其中静态页面基于 React 框架，动态服务基于 Rust，封装在 docker 容器中运行。</p>
      <p>如果喜欢的话，可以在下面点赞哦，如果不喜欢的话也可以点踩哦，欢迎留言～</p>
    </div>
  );

  let buttons = (
    <ButtonsGroup>
      <LikeButton />
      <DislikeButton />
      <Placeholder />
      <CommentHead />
    </ButtonsGroup>
  );

  let comment = <Comment />;

  return (
    <div className={app}>
      {header}
      {content}
      {buttons}
      {comment}
    </div>
  );
}

export default App;
