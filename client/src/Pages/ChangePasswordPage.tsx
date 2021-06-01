import * as React from 'react';

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
      <h1>Change Password</h1>
      <form onSubmit={onSubmit}>
        Username: <input type="text" value={username} onChange={(e) => setUsername(e.target.value)} /><br/>
        Current Password <input type="password" value={currentPassword} onChange={e => setCurrentPassword(e.target.value)} /> <br/>
        New Password <input type="password" value={newPassword} onChange={e => setNewPassword(e.target.value)} /> <br/>
        Confirm New Password <input type="password" value={confirmNewPassword} onChange={e => setConfirmNewPassword(e.target.value)} /> <br/>
        <button type="submit">Change Password</button>
      </form>
    </>
  )
};

export default ChangePasswordPage;

