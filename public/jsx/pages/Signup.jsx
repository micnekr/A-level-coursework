const Form = ReactBootstrap.Form;

function Signup() {
  // Have variables to keep track of the state
  const [username, set_username] = useState("");
  const [password, set_password] = useState("");

  // em is short for "error message"
  const [username_em, set_username_em] = useState("");
  const [password_em, set_password_em] = useState("");
  const [overall_em, set_overall_em] = useState("");

  // Remove the error messages when typing
  useEffect(() => {
    set_username_em("");
    set_password_em("");
    set_overall_em("");
  }, [username, password]);

  return <PageContainerBox title="Sign up">
    <Form>
      <Form.Group className="mb-3" controlId="formBasicUsername">
        <Form.Label>Username</Form.Label>
        <Form.Control type="text" placeholder="Username" value={username} onChange={e => {
          const new_username = e.target.value;
          // Do not allow usernames that are too long
          if (new_username.length > 50) return set_username_em("The username should not be longer than 50 symbols");
          set_username(new_username);
        }} />
        <ErrorMessage em={username_em} />
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicPassword">
        <Form.Label>Password</Form.Label>
        <Form.Control type="password" placeholder="Password" value={password} onChange={e => {
          const new_password = e.target.value;
          set_password(new_password);
        }} />
        <ErrorMessage em={password_em} />
      </Form.Group>

      <PasswordStrength password={password} />

      <ErrorMessage em={overall_em} />
      <Button variant="primary" onClick={submit}>
        Sign up
      </Button>
    </Form>
  </PageContainerBox>;


  async function submit() {
    // Check that a username and a password were supplied
    if (username === "") return set_username_em("Please specify a username");
    if (password === "") return set_password_em("Please specify a password");
    // Check if there are error messages regarding the username or password and avoid submitting the form in that case
    if (password_em !== "" || username_em !== "") return;
    // Make the actual request
    const res = await f("/api/signup", "POST", {
      username, password
    });

    // if it was not successful, show the error message
    if (res.status >= 400) {
      // Read the error message
      const error = await res.text();
      return set_overall_em(error);
    }

    // Otherwise, refirect to the calendar page
    window.location.href = "/";
  }
}
