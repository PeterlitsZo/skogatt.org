import React from 'react';
import axios from 'axios';

import {ReactComponent as Dislike} from '../svg/thumbs-down.svg'

export default class DislikeButton extends React.Component {
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
      <div className="button" onClick={this.addOne}>
        <Dislike className="icon" />
        Disike({isNaN(this.state.dislike_number) ? '...' : this.state.dislike_number})
      </div>
    );
  }
}

