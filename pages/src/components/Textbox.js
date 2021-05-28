import React from 'react';

import {ReactComponent as Text} from '../svg/type.svg';
import {ReactComponent as Submit} from '../svg/send.svg';

export default class Textbox extends React.Component {
  constructor(props) {
    super(props);
    this.textArea = React.createRef();
    this.focusTextArea = this.focusTextArea.bind(this);
  }
  focusTextArea() {
    this.textArea.current.focus();
  }
  render() {
    return (
      <div className="textbox">
        <div className="title" onClick={this.focusTextArea}>
          <Text className="icon" />
          Text for submit:
        </div>
        <textarea ref={this.textArea} className="textarea" />
        <div className="buttons bottom">
          <div className="none"/>
          <div className="dark-button">
            <Submit className="icon" />
            <span className="text">Submit</span>
          </div>
        </div>
      </div>
    );
  }
}
