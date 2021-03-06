import React from 'react';
import axios from 'axios';

import {ReactComponent as Dislike} from '../svg/thumbs-down.svg'
import {ReactComponent as Like} from '../svg/thumbs-up.svg';
import {ReactComponent as IDontKnow} from '../svg/more-horizontal.svg';

import {forPhone, forDevice} from './ButtonsGroup.module.scss';
import {button, darkButton, noBorder} from './Button.module.scss';

export class Button extends React.Component {
  render() {
    let buttonClass = this.props.dark ? darkButton : button;
    if (this.props.className) {
      buttonClass += ' ' + this.props.className;
    }
    if(this.props.forPhone) {
      buttonClass += ' ' + forPhone;
    }
    if(this.props.forDevice) {
      buttonClass += ' ' + forDevice;
    }
    if(this.props.noBorder) {
      buttonClass += ' ' + noBorder;
    }
    if(this.props.disable) {
      console.log("fuck");
    }

    let children = this.props.children;
    let clickFunction = this.props.clickFunction;

    return (
      <button
        className={buttonClass}
        onClick={clickFunction}
        disabled={this.props.disable}
      >
        {children}
      </button>
    )
  }
}

export class DislikeButton extends React.Component {
  constructor(props) {
    super(props);
    this.state = {dislike_number: NaN};
    this.addOne = this.addOne.bind(this);

    // get data from server
    axios.get('/api/v1/home/dislike')
      .then(response => {
        this.setState({
          dislike_number: response.data.dislike
        });
      })
      .catch(error => {
        console.log("Dislike Button", error);
      });
  }
    
  addOne() {
    // If get data(this.state.dislike_number is not NaN), then +1.
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
    // render a button that show this.state.dislike_number. If clicked, then +1.
    let infomation_number;
    if(isNaN(this.state.dislike_number)) {
        infomation_number = <IDontKnow />;
    } else {
        infomation_number = <span>{this.state.dislike_number}</span>;
    }

    return (
      <Button clickFunction={this.addOne}>
        <Dislike />
        <span className={forDevice}>Dislike(</span>
          {infomation_number}
        <span className={forDevice}>)</span>
      </Button>
    );
  }
}


export class LikeButton extends React.Component {
  constructor(props) {
    super(props);
    this.state = {like_number: NaN};
    this.addOne = this.addOne.bind(this);

    // get data from server
    axios.get('/api/v1/home/like')
      .then(response => {
        this.setState({
          like_number: response.data.like
        });
      })
      .catch(error => {
        console.log("Like Button: ", error);
      });
  }

  addOne() {
    // If get data(this.state.dislike_number) is not NaN, then +1.
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
    // render a button that show this.state.dislike_number. If clicked, then +1.
    let infomation_number;
    if(isNaN(this.state.like_number)) {
        infomation_number = <IDontKnow />;
    } else {
        infomation_number = <span>{this.state.like_number}</span>;
    }

    return (
      <Button clickFunction={this.addOne}>
        <Like />
        <span className={forDevice}>Like(</span>
          {infomation_number}
        <span className={forDevice}>)</span>
      </Button>
    );
  }
}
