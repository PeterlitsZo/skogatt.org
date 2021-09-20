import React from 'react';

import {buttonsGroup, placeholder, tag, tagline, info, forPhone, forDevice} from './ButtonsGroup.module.scss';

// ButtonsGroup: the group of a lot of buttons(see `./Button.js`), text, and so
// on.
export class ButtonsGroup extends React.Component {
  render() {
    let children = this.props.children;
    let buttonsGroupClass = buttonsGroup;
    if (this.props.className) {
      buttonsGroupClass += ' ' + this.props.className;
    }

    return (
      <div className={buttonsGroupClass}>
        {children}
      </div>
    );
  }
}

// Placeholder: a variable-length space.
export class Placeholder extends React.Component {
  render() {
    return <span className={placeholder} />;
  }
}

// Tag: show tags for something below.
export class Tag extends React.Component {
  render() {
    let withLine = this.props.withLine;
    let children = this.props.children;
    let className = tag;
    if (this.props.className) {
      className += ' ' +  this.props.className;
    }

    let line = null;
    if (withLine) {
      line = <span className={tagline} />;
    }

    return (
      <span className={className}>
        {children}
        {line}
      </span>
    );
  }
}

// Info: show text.
export class Info extends React.Component {
  render() {
    let children = this.props.children;

    let infoClass = info;
    if(this.props.forPhone) {
      infoClass += ' ' + forPhone;
    }
    if(this.props.forDevice) {
      infoClass += ' ' + forDevice;
    }

    return (
      <span className={infoClass} >
        {children}
      </span>
    );
  }
}
