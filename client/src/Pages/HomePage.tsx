import React from 'react';
import { Link } from '../Components/Link';
import { changePasswordRoute, homePageRoute } from '../routes';

const HomePage = () => {
  return (
    <div>
      <Link to={homePageRoute()}>Objstor</Link>&nbsp;
      <Link to={changePasswordRoute()}>Change Password</Link>
    </div>
  );
}

export default HomePage;
