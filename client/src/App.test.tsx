import React from 'react';
import { render, screen } from '@testing-library/react';
import { App } from './App';

test('renders text', () => {
  render(<App />);
  const text = screen.getByText(/Welcome to objstor!/i);
  expect(text).toBeInTheDocument();
});
