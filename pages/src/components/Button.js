import React from 'react';
import axios from 'axios';

import {ReactComponent as Dislike} from '../svg/thumbs-down.svg'
import {ReactComponent as Like} from '../svg/thumbs-up.svg';

export class DislikeButton extends React.Component {
  constructor(props) {
    super(props);
    this.state = {dislike_number: NaN};

    this.addOne = this.addOne.bind(this);
    axios.get('/api/v1/home/dislike')
      .then(response => {
        console.log(response);
        this.setState({
          dislike_number: response.data.dislike
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
      <div className="button" onClick={this.addOne}>
        <Dislike className="icon" />
        <span className="text">Disike(</span>
        {isNaN(this.state.dislike_number) ? '...' : this.state.dislike_number}
        <span className="text">)</span>
      </div>
    );
  }
}


export class LikeButton extends React.Component {
  constructor(props) {
    super(props);
    this.state = {like_number: NaN};

    this.addOne = this.addOne.bind(this);
    axios.get('/api/v1/home/like')
      .then(response => {
        console.log(response);
        this.setState({
          like_number: response.data.like
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
      <div className="button" onClick={this.addOne}>
        <Like className="icon" />
        <span className="text">Like(</span>
        {isNaN(this.state.like_number) ? '...' : this.state.like_number}
        <span className="text">)</span>
      </div>
    );
  }
}
