import React from 'react';
import axios from 'axios';

import {ReactComponent as Like} from '../svg/thumbs-up.svg';

export default class LikeButton extends React.Component {
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


