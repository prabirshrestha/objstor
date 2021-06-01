# objstor

One stop for storing all your data.

# Running objstor in watch mode

```bash
yarn install
yarn start
```

Open `http://127.0.0.1:5000` in browser to view the app.

# Building objstor for release

```bash
yarn install
yarn build
```

# Help

```bash
objstor --help
```

# Running objstor server

```bash
objstor serve
```

# APIs

## Change Password

```bash
curl -X POST -i 'http://127.0.0.1:5000/api/password' --data '{"username":"admin","current_password": "admin", "new_password": "admin1"}'
```

# LICENSE

MIT
