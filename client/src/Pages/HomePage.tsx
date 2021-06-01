import React from 'react';
import { Link } from '../Components/Link';

const HomePage = () => {
  return (
    <div>
      <Link to="/">Objstor</Link>&nbsp;
      <Link to="/changepassword">Change Password</Link>
    </div>
  );
}

export default HomePage;
