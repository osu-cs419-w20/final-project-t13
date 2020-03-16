import React from 'react'
import ReactDOM from 'react-dom'
import { Provider } from 'react-redux'
import { BrowserRouter } from 'react-router-dom'
import Modal from 'react-modal'

import App from './app'
import store from './store'

import './reset.css'
import './styles.css'

Modal.setAppElement('.app-root')

ReactDOM.render(
  <Provider store={store}>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </Provider>,
  document.querySelector('.app-root')
)
