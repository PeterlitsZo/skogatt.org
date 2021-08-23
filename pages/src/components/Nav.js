import React from 'react';

import {ButtonsGroup, Info, Placeholder} from './ButtonsGroup';
import {Button} from './Button';

import {nav} from './Nav.module.scss';

export class Nav extends React.Component {
  render() {
    return (
      <ButtonsGroup className={nav}>
        <Info>skogkatt.org</Info>
        <Placeholder />
        <Button>Sign in</Button>
      </ButtonsGroup>
    );
  }
}
