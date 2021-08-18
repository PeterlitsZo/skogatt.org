import React from 'react';

import {ReactComponent as Text} from '../svg/type.svg';
import {ReactComponent as Submit} from '../svg/send.svg';
import {ReactComponent as Loading} from '../svg/loader.svg';
import {ReactComponent as Loading3s} from '../svg/loading-3s.svg';

import {Button} from './Button';
import {ButtonsGroup, Info, Placeholder} from './ButtonsGroup';

import {textbox} from './Textbox.module.scss';

export class Textbox extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      state: 'OK', // state set: {'OK', 'waiting'}
      text: '',
      warning_info: false,
      warning_refresh_key: 0,
    };

    this.textArea = React.createRef();

    this.focusTextArea = this.focusTextArea.bind(this);
    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);

    this.refresh = props.refresh;
    this.submit = props.submit;

    this.before_info_timeout_handle = null;
  }

  focusTextArea() {
    this.textArea.current.focus();
  }

  handleChange(event) {
    this.setState({
      text: event.target.value
    });
  }

  async handleSubmit() {
    this.setState({state: 'waiting'});

    const flag = await this.submit(this.state.text);
    // if success, then set text to empty to wait for next operation.
    if (flag) {
      this.setState({text: ''});
      this.refresh();
    // else show warning
    } else {
      this.setState({error_log: true});
      if (this.before_info_timeout_handle) {
        console.log("clear timeout handle");
        clearTimeout(this.before_info_timeout_handle);
        this.before_info_timeout_handle = null;
        this.setState({warning_refresh_key: this.state.warning_refresh_key + 1});
      }
      this.before_info_timeout_handle = setTimeout(() => {
        console.log("set timeout handle");
        this.setState({error_log: false});
      }, 3000);
    }

    this.setState({state: 'OK'});
  }

  render() {
    // Title of the textarea.
    let title = (
      <ButtonsGroup onClick={this.focusTextArea}>
        <Info><Text />Text for submit:</Info>
      </ButtonsGroup>
    );

    // Main part of textarea.
    let textarea = (
      <textarea
        ref={this.textArea}
        className="textarea"
        value={this.state.text}
        onChange={this.handleChange}
      />
    );

    // Warning. Show if user's operation is too frequent.
    let warning = (
      <Warning
        msg={"Warning! The operation is too frequent"}
        phoneMsg={"Too frequent"}
        show={this.state.error_log}
        refreshKey={this.state.warning_refresh_key}
      />
    );

    // Submit button
    let submitButtonIcon;
    if (this.state.state === 'OK') {
      submitButtonIcon = <Submit />;
    } else {
      submitButtonIcon = <Loading />;
    }
    let submitButton = (
      <Button dark clickFunction={this.handleSubmit}>
        {submitButtonIcon}
        <span className="text">Submit</span>
      </Button>
    );

    return (
      <div className={textbox}>
        {title}
        {textarea}
        <ButtonsGroup>
          <Placeholder />
          {warning}
          {submitButton}
        </ButtonsGroup>
      </div>
    );
  }
}

// `Waring` component used by `Textbox`.
class Warning extends React.Component {
  render() {
    let props = this.props
    if (props.show) {
      return (
        <Info>
          <Loading3s key={props.refreshKey} />
          <Info forDevice>{props.msg}</Info>
          <Info forPhone>{props.phoneMsg}</Info>
        </Info>
      );
    } else {
      return null;
    }
  }
}
