// Imports
const Button = ReactBootstrap.Button;
const Form = ReactBootstrap.Form;

const useState = React.useState;
const useEffect = React.useEffect;

function Login() {
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

    return <PageContainerBox title="Log in">
        <Form>
          // The username input
            <Form.Group className="mb-3" controlId="formBasicUsername">
                <Form.Label>Username</Form.Label>
                <Form.Control type="text" placeholder="Username" value={username} onChange={e => set_username(e.target.value)} />
                <ErrorMessage em={username_em} />
            </Form.Group>

          // The password input
            <Form.Group className="mb-3" controlId="formBasicPassword">
                <Form.Label>Password</Form.Label>
                <Form.Control type="password" placeholder="Password" value={password} onChange={e => set_password(e.target.value)} />
                <ErrorMessage em={password_em} />
            </Form.Group>
          // This error message shows up if there is an error message received from the server
            <ErrorMessage em={overall_em} />
          // Submit button
            <Button variant="primary" onClick={submit}>
                Log in
            </Button>
        </Form>
    </PageContainerBox>;


    async function submit() {
        // Check that a username and a password were supplied
        if (username === "") return set_username_em("Please specify a username");
        if (password === "") return set_password_em("Please specify a password");
        // Make the actual request
        const res = await f("/api/login", "POST", {
            username, password
        });

        // if it was not successful, show the error message
        if (res.status >= 400) {
            // Read the error message
            const error = await res.text();
            return set_overall_em(error);
        }


        // Otherwise, refirect to the calendar page
        window.location.href = "/calendar";
    }
}
