const Button = ReactBootstrap.Button;
const Form = ReactBootstrap.Form;

const useState = React.useState;

function Login() {
  // Have variables to keep track of the state
  const [username, set_username] = useState("");
  const [password, set_password] = useState("");

  return <PageContainerBox title="Log in">
    <Form>
      <Form.Group className="mb-3" controlId="formBasicUsername">
        <Form.Label>Username</Form.Label>
        <Form.Control type="text" placeholder="Username" value={username} onChange={e => set_username(e.target.value)} />
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicPassword">
        <Form.Label>Password</Form.Label>
        <Form.Control type="password" placeholder="Password" value={password} onChange={e => set_password(e.target.value)} />
      </Form.Group>
      <Button variant="primary" onClick={submit}>
        Log in
      </Button>
    </Form>
  </PageContainerBox>;


  function submit() { }
}
