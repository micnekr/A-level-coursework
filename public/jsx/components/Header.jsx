const Button = ReactBootstrap.Button;
const Image = ReactBootstrap.Image;

/** The component displayed at the top of the screen, like a navbar
 */
function Header() {
  const [is_logged_in, set_is_logged_in] = useState(false);
  const [notifications, set_notifications] = useState([]);

  /// Request an endpoint and call the setter with the data returned
  async function request(endpoint, setter) {
    const res = await f(endpoint, "GET");

    // Log the error if something went wrong and do not proceed
    if (res.status >= 400) return console.error("Error within a response:", res);

    const data = await res.json();

    setter(data);
  }

  // Check if logged in when the page is loaded and load notification count
  useEffect(() => {
    request("/api/is_logged_in", set_is_logged_in);
    request("/api/get_notifications", set_notifications);
  }, []);

  // A list of buttons to show only when the user is logged in
  const buttons_shown_when_logged_in = [
    <HeaderButton is_filled={true} content="Calendar" link="/" key={0} />,
    <HeaderButton content="Friends" link="/friends" key={1} />,
  ];
  // A list of buttons to show only when the user is logged out
  const buttons_shown_when_logged_out = [
    <HeaderButton is_filled={true} content="Login" link="/login" key={0} />,
    <HeaderButton content="Signup" link="/signup" key={1} />
  ];

  return <div className="container-fluid" style={{
    backgroundColor: "#FAFAFA"
  }}>
    <div className="row container-md mx-auto p-2 align-items-center justify-content-md-start">
      <div className="row">
        {/* Logo image */}
        <div className="col-3 col-sm-2">
          <Image src="/img/logo.png" height={38} style={{ cursor: "pointer" }} onClick={() => { window.location.href = "/" }
          } />
        </div>
        {/* Links and buttons */}
        <div className="col container">
          <div className="row justify-content-md-evenly">
            {/* Only display these buttons if logged in */}
            {is_logged_in ? buttons_shown_when_logged_in : buttons_shown_when_logged_out}
          </div>
        </div>

        {/* If logged in, show the notifications */}
        {
          is_logged_in ?
            < div className="col-3 col-sm-2 position-relative">
              <Image src="/img/notification.png" height={38} style={{ cursor: "pointer" }} onClick={() => { }
              } />
              {/* If there are notifications, show their number */}
              {notifications.length !== 0 ?
                <NotificationCircle notification_number={notifications.length} />
                : null}
            </div>
            : null
        }
      </div>
    </div>
  </div >;
}

function HeaderButton(props) {
  const { content, link, is_filled } = props;
  return <div className="col-6">
    {/* Select between filled and not */}
    <Button variant={`${is_filled ? "" : "outline-"}primary`} onClick={() => {
      // Redirect to the page
      window.location.href = link;
    }}
      className="mx-auto d-block w-100"
    >
      {content}
    </Button>
  </div>
}

function NotificationCircle(props) {
  let { notification_number } = props;

  // Convert to the string
  notification_number = "" + notification_number;
  // Shorten it if we have more than 2 digits
  if (notification_number.length > 2) notification_number = "99";


  return <div className="position-relative rounded-circle" style={{
    top: "-10px",
    left: "25px",
    height: "24px",
    width: "24px",
    textAlign: "center",
    backgroundColor: "red",
    color: "white"
  }}>{notification_number}</div>;
}
