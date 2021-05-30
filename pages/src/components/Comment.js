import React from 'react';
import axios from 'axios';
import moment from 'moment';

import {ReactComponent as CommentIcon} from '../svg/message-circle.svg';
import {Textbox} from './Textbox';

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
    console.log(this.state);
  }
  render() {
    const content = this.state.content;
    return (
      <div className="item">
        <div className="head">
          {this.props.content.ip}
          <span> - </span>
          {moment(this.props.content.datetime).fromNow()}
        </div>
        <div className="text">{this.props.content.content}</div>
      </div>
    )
  }
}

class CommentsList extends React.Component {
  render() {
    return (
      <div className="list">
        {
          this.props.list
            .reverse()
            .map((data) => <CommentsItem content={data}/>)
        }
      </div>
    );
  }
}

export class Comment extends React.Component {
  constructor(props) {
    super(props);
    this.state = {comments: []};

    this.submit = this.submit.bind(this);
    this.refresh = this.refresh.bind(this);
    this.refresh();
  }
  refresh() {
    axios.get('/api/v1/home/comments')
      .then(response => {
        console.log(response);
        this.setState({
          comments: response.data
        });
      })
      .catch(error => {
        console.error(error);
      });
  }
  // Submit. return value:
  //  - true: submit successfully
  //  - false: submit failly
  async submit(text) {
    console.log('submit: ', text);
    try {
      await axios.post('/api/v1/home/comments', text);
      return true;
    } catch (error) {
      return false;
    }
  }
  render() {
    return (
      <div className="comment">
        <Textbox submit={this.submit} refresh={this.refresh} />
        <CommentsList list={this.state.comments}/>
      </div>
    );
  }
}

