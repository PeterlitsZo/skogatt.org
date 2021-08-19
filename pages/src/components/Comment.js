import React from 'react';
import axios from 'axios';
import moment from 'moment';

import {ReactComponent as CommentIcon} from '../svg/message-circle.svg';
import {ReactComponent as Pages} from '../svg/layers.svg';
import {ReactComponent as To} from '../svg/arrow-right.svg';
import {ReactComponent as Dots} from '../svg/more-horizontal.svg';
import {ReactComponent as Before} from '../svg/chevron-left.svg';
import {ReactComponent as Head} from '../svg/chevrons-left.svg';
import {ReactComponent as After} from '../svg/chevron-right.svg';
import {ReactComponent as Tail} from '../svg/chevrons-right.svg';

import {Textbox} from './Textbox';
import {ButtonsGroup, Tag, Placeholder, Info} from './ButtonsGroup';
import {Button} from './Button';
import {Pagination, PaginationJump} from './Pagination';

import {forDevice} from './ButtonsGroup.module.scss';
import {item, text, list, head, tail, current as currentClass, input, comment, tag} from './Comment.module.scss';

// The head of comments. It is a `Tag` in `ButtonsGroup`.
export class CommentHead extends React.Component {
  render() {
    return (
      <Tag withLine className={tag}>
        <CommentIcon />
        <span>Comments</span>
      </Tag>
    );
  }
}

// The element of comments' list.
class CommentsItem extends React.Component {
  constructor(props) {
    super(props);
    const fold = this.props.content.content.length > 250;

    // `fold`: boolean type:
    //  - true: this `CommentsItem` is folded (with a button to `unfold`).
    //  - false: unfolded (with a button to `fold`).
    this.state = { fold };

    this.changeFoldState = this.changeFoldState.bind(this);
  }

  changeFoldState() {
    this.setState({
      fold: !this.state.fold,
    });
  }

  render() {
    /* type: {ip: string, datetime: string, content: string} */
    const content = this.props.content;
    const needFold = content.content.length > 250;

    let fold_button = null;
    let contentText = content.content;

    // If need fold content, then fold content and and `Fold/Unfold` button.
    if (needFold) {
      fold_button = (
        <Button dark clickFunction={ this.changeFoldState }>
          { this.state.fold ? 'Unfold' : 'Fold' }
        </Button>
      );
      if (this.state.fold)
        contentText = contentText.substring(0, 250) + '...';
    }

    // split text with newline token(`\n`), and then render those to `<p>`
    // elements.
    contentText = contentText.split('\n').map((i, key) => {
      return <p key={key}>{i}</p>
    });

    return (
      <div className={item}>
        <ButtonsGroup>
          <Info>{ content.ip }</Info>
          <Info> - </Info>
          <Info>{ moment(content.datetime).fromNow() }</Info>
          <Placeholder />
          { fold_button }
        </ButtonsGroup>
        <div className={text}>{ contentText }</div>
        <ButtonsGroup>{ fold_button }</ButtonsGroup>
      </div>
    );
  }
}

class CommentsList extends React.Component {
  render() {
    // main part - comments
    const commentsList = this.props.comments.list
      .map((data) => {
        return <CommentsItem content={data} key={ data.id } />
      });

    // pagination of the comments
    const commentsListPagination = className => (
      this.props.comments.length > 1
      ? <Pagination
          current={this.props.comments.current}
          length={this.props.comments.length}
          handle={this.props.handle}
          className={className}
        />
      : null);

    return (
      <div className={list}>
        { commentsListPagination(head) }
        <div>{ commentsList }</div>
        { commentsListPagination(tail) }
      </div>
    );
  }
}

export class Comment extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      comments: {
        list: [],
        current: 1,
        length: 1,
      },
    };

    this.submit = this.submit.bind(this);
    this.refresh = this.refresh.bind(this);

    this.refresh(1);
  }

  // Post a request to server and update the state.
  refresh(page) {
    page = parseInt(page)
    if(page < 1) page = 1;
    axios.get(`/api/v1/home/comments?page=${page}`)
      .then(response => {
        console.log(response);
        let length = parseInt(response.data.length)
        if(page > length) {
          this.refresh(length);
          return;
        }
        this.setState({
          comments: {
            list: JSON.parse(response.data.result),
            current: page,
            length,
          }
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
    try {
      await axios.post('/api/v1/home/comments', text);
      return true;
    } catch (error) {
      return false;
    }
  }

  render() {
    return (
      <div className={comment}>
        <Textbox submit={this.submit} refresh={() => this.refresh(1)} />
        <CommentsList
          handle={this.refresh}
          comments={this.state.comments}
        />
      </div>
    );
  }
}
