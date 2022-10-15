const Button = ReactBootstrap.Button;
const Form = ReactBootstrap.Form;

const useState = React.useState;

function Signup() {
  // Have variables to keep track of the state
  const [username, set_username] = useState("");
  const [password, set_password] = useState("");

  return <PageContainerBox title="Sign up">
    <Form>
      <Form.Group className="mb-3" controlId="formBasicUsername">
        <Form.Label>Username</Form.Label>
        <Form.Control type="text" placeholder="Username" value={username} onChange={e => set_username(e.target.value)} />
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicPassword">
        <Form.Label>Password</Form.Label>
        <Form.Control type="password" placeholder="Password" value={password} onChange={e => set_password(e.target.value)} />
      </Form.Group>

      <PasswordStrength password={password} />

      <Button variant="primary" onClick={submit}>
        Sign up
      </Button>
    </Form>
  </PageContainerBox>;


  function submit() { }
}
