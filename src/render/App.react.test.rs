use React from 'react';
use ReactDOM from 'react-dom';
use App from 'crate::App.react';
use { Provider } from 'react-redux';
use Store from '../redux/Store';

it('renders without crashing', () => {
  const div = document.createElement('div');
  ReactDOM.render(<Provider store={Store}><App /></Provider>, div);
});
