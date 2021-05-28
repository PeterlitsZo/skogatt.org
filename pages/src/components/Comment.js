import React from 'react';
import axios from 'axios';
import moment from 'moment';

import {ReactComponent as CommentIcon} from '../svg/message-circle.svg';
import Textbox from './Textbox';

export class CommentHead extends React.Component {
  render() {
    return (
      <div className="tab">
        <CommentIcon className="icon"/>
        <span className="text">Comments</span>
        <div className="tabline"/>
      </div>
    )
  }
}

class CommentsItem extends React.Component {
    constructor(props) {
      super(props);

      this.state = {
        content: props.content
      };
    }
    render() {
      const content = this.state.content;
      return (
        <div className="item">
          <div className="head">
            {content.ip} - {moment(content.datetime).fromNow()}
          </div>
          <div className="text">{content.content}</div>
        </div>
      )
    }
}

class CommentsList extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      list: props.list
    };
  }
  render() {
    return (
      <div className="list">
        {this.state.list.map((data) => <CommentsItem content={data}/>)}
      </div>
    );
  }
}

export class Comment extends React.Component {
  constructor(props) {
    super(props);
  }
  render() {
    return (
      <div className="comment">
        <Textbox />
        <CommentsList list={
          [
            {ip: "127.0.0.1", content: "Hello World", datetime: moment('2001-5-3').format()},
            {ip: "127.0.0.1", content: "Hello World", datetime: moment().format()},
            {ip: "127.0.0.1", content: "Hello World", datetime: moment().format()},
            {ip: "127.0.0.1", content: "Hello World", datetime: moment().format()},
            {ip: "127.0.0.1", content: "Hello World", datetime: moment().format()},
          ]
        }/>
      </div>
    );
  }
}

