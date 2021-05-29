import React from 'react';

import {ReactComponent as Text} from '../svg/type.svg';
import {ReactComponent as Submit} from '../svg/send.svg';

export default class Textbox extends React.Component {
  constructor(props) {
    super(props);
    this.state = {text: ''};

    this.textArea = React.createRef();
    this.form = React.createRef();

    this.focusTextArea = this.focusTextArea.bind(this);
    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
    this.refresh = props.refresh;
    this.submit = props.submit;
  }
  focusTextArea() {
    this.textArea.current.focus();
  }
  handleChange(event) {
    console.log(event);
    this.setState({
      text: event.target.value
    });
  }
  handleSubmit() {
    this.submit(this.state.text);
    this.setState({text: ''});
    this.refresh();
  }
  render() {
    return (
      <div
        className="textbox"
        ref={this.form}
      >
        <div className="title" onClick={this.focusTextArea}>
          <Text className="icon" />
          Text for submit:
        </div>
        <textarea
          ref={this.textArea}
          className="textarea"
          value={this.state.text}
          onChange={this.handleChange}
        />
        <div className="buttons bottom">
          <div className="none"/>
          <div className="dark-button" onClick={this.handleSubmit}>
            <Submit className="icon" />
            <span className="text">Submit</span>
          </div>
        </div>
      </div>
    );
  }
}
