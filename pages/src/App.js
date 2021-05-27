import React from 'react';
import axios from 'axios';

import './App.scss';
import {ReactComponent as Like} from './svg/thumbs-up.svg'
import {ReactComponent as Dislike} from './svg/thumbs-down.svg'


class LikeButtom extends React.Component {
  constructor(props) {
    super(props);
    this.state = {like_number: NaN};

    this.addOne = this.addOne.bind(this);
    axios.get('/api/v1/home/like')
      .then(response => {
        console.log(response);
        this.setState({
          like_number: response.data
        });
      })
      .catch(error => {
        console.log(error);
      });
  }
  addOne() {
    this.setState(state => {
      if (isNaN(state.like_number)) {
        return {}
      } else {
        axios.post('/api/v1/home/like');
        return {
          like_number: state.like_number + 1
        }
      }
    });
  }
  render() {
    return (
      <div className="buttom" onClick={this.addOne}>
        <Like className="icon" />
        Like({isNaN(this.state.like_number) ? '...' : this.state.like_number})
      </div>
    );
  }
}


class DislikeButtom extends React.Component {
  constructor(props) {
    super(props);
    this.state = {dislike_number: NaN};

    this.addOne = this.addOne.bind(this);
    axios.get('/api/v1/home/dislike')
      .then(response => {
        console.log(response);
        this.setState({
          dislike_number: response.data
        });
      })
      .catch(error => {
        console.log(error);
      });
  }
  addOne() {
    this.setState(state => {
      if (isNaN(state.dislike_number)) {
        return {}
      } else {
        axios.post('/api/v1/home/dislike');
        return {
          dislike_number: state.dislike_number + 1
        }
      }
    });
  }
  render() {
    return (
      <div className="buttom" onClick={this.addOne}>
        <Dislike className="icon" />
        Disike({isNaN(this.state.dislike_number) ? '...' : this.state.dislike_number})
      </div>
    );
  }
}

function App() {
  return (
    <div className="app">
      <header className="header">
        <div className="logo">
          <p>Peterlits</p><p className="cursor" />
        </div>
        <div className="footnote">
          <p>生活 · 技术 · 存在 · 我</p><p className="line" /><p>●</p>
        </div>
      </header>
      <div className="content">
        <p>Peterlits 其人，本名周泓余，现于苏州大学计算机科学与技术系在读。</p>
        <p>这里是 Peterlits 的小网站，它基于 nignx 反向代理，其中静态页面基于 React 框架，动态服务基于 Rust，封装在 docker 容器中运行。</p>
        <p>如果喜欢的话，可以在下面点赞哦，如果不喜欢的话也可以点踩哦，欢迎留言～</p>
      </div>
      <LikeButtom />
      <DislikeButtom />
    </div>
  );
}

export default App;
