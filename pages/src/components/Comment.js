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
    console.log(this.props.comments);
    return (
      <div className="list">
        {
          this.props.comments.length > 1
            ? <PaperNav
                current={this.props.comments.current}
                length={this.props.comments.length}
                handle={this.props.handle}
              />
            : null
        }
        <div>
          {
            this.props.comments.list
              .map((data) => <CommentsItem content={data}/>)
          }
        </div>
        {
          this.props.comments.length > 1
            ? <PaperNav
                current={this.props.comments.current}
                length={this.props.comments.length}
                handle={this.props.handle}
              />
            : null
        }
      </div>
    );
  }
}

export class PaperNav extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      pageNumber: '',
    };
  }

  render() {
    let result = [];
    const handle = this.props.handle;
    const current = this.props.current;
    const length = this.props.length;

    // The button that will jump to `paperNum` paper by `handle` function
    function PaperNavButton(props) {
      return (
        <div
          className={'dark-button' + (props.forPhone ? '' : ' notForPhone')}
          onClick={() => handle(props.paperNum)}
        >
          {props.paperNum}
        </div>
      );
    }

    // Display the buttons before `current` button
    if (this.props.current - 2 > 2) {
      result.push(
        <PaperNavButton paperNum={1} forPhone={false}/>,
        <Dots className="icon notForPhone" />,
        <PaperNavButton paperNum={current - 2} forPhone={false}/>,
        <PaperNavButton paperNum={current - 1} forPhone={false}/>,
      );
    } else {
      for (let i = 1; i < current; i ++)
        result.push(<PaperNavButton paperNum={i} forPhone={false}/>);
    }
    if (current > 1) {
      if (current > 2) {
        result.push(<div className='dark-button forPhone' onClick={() => handle(1)}>
          <Head className='icon' />
        </div>);
      }
      result.push(<div className='dark-button forPhone' onClick={() => handle(current - 1)}>
        <Before className='icon' />
      </div>);
    }

    // Display the `current` button
    result.push(
      <span className="current">{current}</span>,
      <span className="forPhone">/ {length}</span>,
    );

    // Display the buttons after `current` button
    if (this.props.current + 2 < this.props.length - 1) {
      result.push(
        <PaperNavButton paperNum={current + 1} forPhone={false}/>,
        <PaperNavButton paperNum={current + 2} forPhone={false}/>,
        <Dots className="icon notForPhone" />,
        <PaperNavButton paperNum={length} forPhone={false}/>,
      );
    } else {
      for (let i = this.props.current + 1; i <= this.props.length; i ++)
        result.push(<PaperNavButton paperNum={i} forPhone={false}/>);
    }
    if (current < length) {
      result.push(<div className='dark-button forPhone' onClick={() => handle(current + 1)}>
        <After className="icon"/>
      </div>);
      if (current < length - 1) {
        result.push(<div className='dark-button forPhone' onClick={() => handle(length)}>
          <Tail className='icon' />
        </div>);
      }
    }

    // Display the blank between buttons and `Jump` component
    result.push(<div className="none" />);

    // Display the `Jump` component
    result.push(
      <form className="buttons notForPhone" onSubmit={(event) => {
        this.props.handle(this.state.pageNumber);
        this.setState({ pageNumber: '' });
        console.log("fuck 2", this.state.pageNumber);
        event.preventDefault();
      }}>
        <label>
          <input type="text" name="page" onChange={(event) => {
            this.setState({ pageNumber: event.target.value });
            console.log("fuck 1", event.target.value);
          }} value={this.state.pageNumber} placeholder="Page" />
        </label>
        <button className="dark-button">
          <To className="icon" />
          Jump
        </button>
      </form>
    );

    // Render the buttons
    return (
      <div className="buttons nav">
        <Pages className="icon" />
        {result.length ? result : null}
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
    axios.get(`/api/v1/home/comments?page=${page}`)
      .then(response => {
        console.log(response);
        this.setState({
          comments: {
            list: JSON.parse(response.data.result),
            current: page,
            length: JSON.parse(response.data.length),
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
      <div className="comment">
        <Textbox submit={this.submit} refresh={() => this.refresh(1)} />
        <CommentsList
          handle={this.refresh}
          comments={this.state.comments}
        />
      </div>
    );
  }
}

