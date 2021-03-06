import { Heading, FormControl, FormLabel, Input, Button } from '@chakra-ui/react';
import * as React from 'react';
import { Link } from '../Components/Link';
import { homePageRoute } from '../routes';

const ChangePasswordPage = () => {
  const [username, setUsername] = React.useState('');
  const [currentPassword, setCurrentPassword] = React.useState('');
  const [newPassword, setNewPassword] = React.useState('');
  const [confirmNewPassword, setConfirmNewPassword] = React.useState('');

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    alert('todo');
  };

  return (
    <>
      <Link to={homePageRoute()}>Objstor</Link>
      <Heading>Change Password</Heading>
      <FormControl onSubmit={onSubmit} isRequired>
        <FormLabel htmlFor="username">Username</FormLabel>
        <Input id="username" type="text" name="username" value={username} onChange={(e) => setUsername(e.target.value)} autoComplete="off" /><br/>
        <FormLabel htmlFor="current_password">Current Password</FormLabel>
        <Input id="current_password" type="password" value={currentPassword} onChange={e => setCurrentPassword(e.target.value)} autoComplete="off"/> <br/>
        <FormLabel htmlFor="new_password">New Password</FormLabel>
        <Input id="new_password" type="password" value={newPassword} onChange={e => setNewPassword(e.target.value)} autoComplete="off" /> <br/>
        <FormLabel htmlFor="confirm_new_password">Confirm New Password</FormLabel>
        <Input id="confirm_new_password" type="password" value={confirmNewPassword} onChange={e => setConfirmNewPassword(e.target.value)} autoComplete="off" /> <br/>
        <Button type="submit">Change Password</Button>
      </FormControl>
    </>
  )
};

export default ChangePasswordPage;

