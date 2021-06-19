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

import {forPhone, forDevice} from './ButtonsGroup.module.scss';
import {item, text, list, head, tail, current as currentClass} from './Comment.module.scss';

// The head of comments. It is a `Tag` in `ButtonsGroup`.
export class CommentHead extends React.Component {
  render() {
    return (
      <Tag withLine>
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
    console.log(this.props.comments);

    const commentsList = this.props.comments.list
      .map((data) => {
        return <CommentsItem content={data} key={ data.id } />
      });

    const commentsListNav = className => (
      this.props.comments.length > 1
      ? <PaperNav
          current={this.props.comments.current}
          length={this.props.comments.length}
          handle={this.props.handle}
          className={className}
        />
      : null);

    console.log(head, tail);
    return (
      <div className={list}>
        { commentsListNav(head) }
        <div>{ commentsList }</div>
        { commentsListNav(tail) }
      </div>
    );
  }
}

class PaperNavJump extends React.Component {
  constructor(props) {
    super(props);

    this.handle = props.handle;

    this.state = { pageNumber: '' };

    this.changePage = this.changePage.bind(this);
    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  changePage() {
    this.handle(this.state.pageNumber);
  }

  handleChange(event) {
    this.setState({
      pageNumber: event.target.value,
    });
  }

  handleSubmit(event) {
    this.props.handle(this.state.pageNumber);
    this.setState({ pageNumber: '' });
    event.preventDefault();
  }

  render() {
    return (
      <from className={forDevice} onSubmit={this.handleSubmit}>
        <ButtonsGroup>
          <input type="text" name="page"
            onChange={this.handleChange}
            value={this.state.pageNumber}
            placeholder="Page"
          />
          <Button dark clickFunction={this.changePage}>
            <To /><span>Jump</span>
          </Button>
        </ButtonsGroup>
      </from>
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

    function mkHandle(paperNum) {
      return () => handle(paperNum);
    }

    // The button that will jump to `paperNum` paper by `handle` function
    function PaperNavButton(props) {
      const forPhone = props.forPhone;
      const forDevice = props.forDevice;

      return (
        <Button dark forPhone={forPhone} forDevice={forDevice}
          clickFunction={mkHandle(props.paperNum)}
        >
          {props.paperNum}
        </Button>
      );
    }

    // Display the buttons before `current` button
    if (this.props.current - 2 > 2) {
      result.push(
        <PaperNavButton paperNum={1} forDevice />,
        <Dots className={forDevice} />,
        <PaperNavButton paperNum={current - 2} forDevice />,
        <PaperNavButton paperNum={current - 1} forDevice />,
      );
    } else {
      for (let i = 1; i < current; i ++)
        result.push(<PaperNavButton paperNum={i} forDevice />);
    }
    if (current > 2) {
      result.push(
        <Button dark forPhone clickFunction={mkHandle(1)}>
          <Head />
        </Button>
      );
    }
    if (current > 1) {
      result.push(
        <Button dark forPhone clickFunction={mkHandle(current - 1)}>
          <Before />
        </Button>
      );
    }

    // Display the `current` button
    result.push(
      <span className={currentClass}>{current}</span>,
      <Info forPhone>/ {length}</Info>,
    );

    // Display the buttons after `current` button
    if (this.props.current + 2 < this.props.length - 1) {
      result.push(
        <PaperNavButton paperNum={current + 1} forDevice />,
        <PaperNavButton paperNum={current + 2} forDevice />,
        <Dots className={forDevice} />,
        <PaperNavButton paperNum={length} forDevice />,
      );
    } else {
      for (let i = this.props.current + 1; i <= this.props.length; i ++)
        result.push(<PaperNavButton paperNum={i} forDevice />);
    }
    if (current < length) {
      result.push(
        <Button dark forPhone clickFunction={mkHandle(current + 1)}>
          <After />
        </Button>
      );
    }
    if (current < length - 1) {
      result.push(
        <Button dark forPhone clickFunction={mkHandle(length)}>
          <Tail />
        </Button>
      );
    }

    // Display the blank between buttons and `Jump` component
    result.push(<Placeholder />);

    // Display the `Jump` component
    result.push(<PaperNavJump handle={handle}/>);

    // Render the buttons
    return (
      <ButtonsGroup className={this.props.className}>
        <Pages />
        {result.length ? result : null}
      </ButtonsGroup>
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
