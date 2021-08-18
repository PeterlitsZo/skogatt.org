import React from 'react';

import {ReactComponent as CommentIcon} from '../svg/message-circle.svg';
import {ReactComponent as Pages} from '../svg/layers.svg';
import {ReactComponent as To} from '../svg/arrow-right.svg';
import {ReactComponent as Dots} from '../svg/more-horizontal.svg';
import {ReactComponent as Before} from '../svg/chevron-left.svg';
import {ReactComponent as Head} from '../svg/chevrons-left.svg';
import {ReactComponent as After} from '../svg/chevron-right.svg';
import {ReactComponent as Tail} from '../svg/chevrons-right.svg';

import {Button} from './Button';
import {ButtonsGroup, Tag, Placeholder, Info} from './ButtonsGroup';
import {forPhone, forDevice} from './ButtonsGroup.module.scss';
import {input, current as current_, paginationButton, jump, from} from './Pagination.module.scss';

class PaginationJump extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      pageNumber: NaN,
    };

    this.input = React.createRef();

    this.focusInput = this.focusInput.bind(this);
    this.handleChange = this.handleChange.bind(this);
    this.handleClick = this.handleClick.bind(this);
    this.handleEnterKey = this.handleEnterKey.bind(this);
  }

  focusInput() {
    this.input.current.focus();
  }

  handleChange(event) {
    let value = event.target.value;

    this.setState({
      pageNumber: parseInt(value),
    });
  }

  handleClick(handle) {
    return e => {
      e.preventDefault();
      handle(this.state.pageNumber);
      this.setState({ pageNumber: NaN });
      this.focusInput();
    }
  }

  handleEnterKey(handle) {
    return e => {
      if(e.nativeEvent.keyCode == 13) {
        handle(this.state.pageNumber);
        this.setState({ pageNumber: NaN });
        this.focusInput();
      }
    }
  }

  render() {
    const {handle} = this.props;
    const showNumber = isNaN(this.state.pageNumber)?'':this.state.pageNumber;

    return (
      <from className={forDevice + ' ' + from}>
        <input type="text" name="page" className={input}
          onChange={this.handleChange}
          onKeyPress={this.handleEnterKey(handle)}
          value={showNumber}
          ref={this.input}
        />
        <Button dark noBorder
          className={paginationButton + ' ' + jump}
          clickFunction={this.handleClick(handle)}
        >
          <To />
        </Button>
      </from>
    );
  }
}

export class Pagination extends React.Component {
  render() {
    let result = [];
    let {handle, current, length, className} = this.props;
    console.log("FUCK", this.props);

    function mkHandle(paperNum) {
      return () => handle(paperNum);
    }

    // The button that will jump to `props.paperNum` paper by `props.handle` function
    function PaginationButton(props) {
      const forPhone = props.forPhone;
      const forDevice = props.forDevice;
      const noBorder = !props.border;
      let className = paginationButton;
      className += (props.className ? ' ' + props.className : '');

      return (
        <Button dark
          forPhone={forPhone} forDevice={forDevice} noBorder={noBorder}
          className={className}
          clickFunction={mkHandle(props.paperNum)}
        >
          {props.paperNum}
        </Button>
      );
    }

    // Display the buttons before `current` button
    if (current - 2 > 2) {
      result.push(
        <PaginationButton paperNum={1} forDevice />,
        <Button dark noBorder disable className={paginationButton} forDevice>
          <Dots className={forDevice} />
        </Button>,
        <PaginationButton paperNum={current - 2} forDevice />,
        <PaginationButton paperNum={current - 1} forDevice />,
      );
    } else {
      for (let i = 1; i < current; i ++)
        result.push(<PaginationButton paperNum={i} forDevice />);
    }
    if (current > 2) {
      result.push(
        <Button dark forPhone noBorder className={paginationButton}
         clickFunction={mkHandle(1)}>
          <Head />
        </Button>
      );
    }
    if (current > 1) {
      result.push(
        <Button dark forPhone noBorder className={paginationButton}
         clickFunction={mkHandle(current - 1)}>
          <Before />
        </Button>
      );
    }

    // Display the `current` button
    result.push(
      <PaginationButton border className={current_} paperNum={current} />,
      <Info forPhone>/ {length}</Info>,
    );

    console.log(current, length);

    // Display the buttons after `current` button
    if (current + 2 < length - 1) {
      console.log("A", current, length);
      result.push(
        <PaginationButton paperNum={current + 1} forDevice />,
        <PaginationButton paperNum={current + 2} forDevice />,
        <Button dark noBorder disable className={paginationButton} forDevice>
          <Dots className={forDevice} />
        </Button>,
        <PaginationButton paperNum={length} forDevice />,
      );
    } else {
      console.log("B", current, length);
      for (let i = current + 1; i <= length; i ++) {
        console.log(i, current, length);
        result.push(<PaginationButton paperNum={i} forDevice />);
      }
    }
    if (current < length) {
      result.push(
        <Button dark forPhone noBorder className={paginationButton}
         clickFunction={mkHandle(current + 1)}>
          <After />
        </Button>
      );
    }
    if (current < length - 1) {
      result.push(
        <Button dark forPhone noBorder className={paginationButton}
         clickFunction={mkHandle(length)}>
          <Tail />
        </Button>
      );
    }

    // Display the blank between buttons and `Jump` component
    result.push(<Placeholder />);

    // Display the `Jump` component
    result.push(<PaginationJump handle={handle}/>);

    // Render the buttons
    return (
      <ButtonsGroup className={className}>
        {result.length ? result : null}
      </ButtonsGroup>
    );
  }
}
