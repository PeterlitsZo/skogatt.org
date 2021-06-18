import React from 'react';

import {buttonsGroup, placeholder, tag, tagline, info, forPhone, forDevice} from './ButtonsGroup.module.scss';

export class ButtonsGroup extends React.Component {
  render() {
    let children = this.props.children;

    return (
      <div className={buttonsGroup}>
        {children}
      </div>
    );
  }
}

export class Placeholder extends React.Component {
  render() {
    return <span className={placeholder} />;
  }
}

export class Tag extends React.Component {
  render() {
    let withLine = this.props.withLine;
    let children = this.props.children;

    let line = null;
    if (withLine) {
      line = <span className={tagline} />;
    }

    return (
      <span className={tag}>
        {children}
        {line}
      </span>
    );
  }
}

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
