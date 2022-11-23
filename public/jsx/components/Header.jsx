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

  return <div className="container-fluid gx-0" style={{
    backgroundColor: "#FAFAFA"
  }}>
    <div className="row container-md mx-auto p-2 align-items-center justify-content-md-start gx-0">
      {/* Logo image */}
      <div className="col-2">
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
          <div className="col-2">
            <NotificationIcon notifications={notifications} />
          </div>
          : null
      }
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

/** A react component to display a circle with the number of notifications
*/
function NotificationCircle(props) {
  let { notification_number } = props;

  // Convert to the string
  notification_number = "" + notification_number;
  // Shorten it if we have more than 2 digits
  if (notification_number.length > 2) notification_number = "99";


  return <div className="position-absolute rounded-circle" style={{
    top: "25px",
    left: "25px",
    height: "24px",
    width: "24px",
    textAlign: "center",
    backgroundColor: "red",
    color: "white"
  }}>{notification_number}</div>;
}

/** A react component to display the bell icon with a notification number
*/
function NotificationIcon(props) {
  const { notifications } = props;
  const notification_number = notifications.length;

  // Should we show the notifications?
  const [show_notifications, set_show_notifications] = useState(true);

  function toggle_show_notification() {
    set_show_notifications(!show_notifications);
  }

  // Center it
  return <div className="text-center">
    <div className="position-relative d-inline-block">
      <div style={{ cursor: "pointer" }} onClick={toggle_show_notification}>
        <Image src="/img/notification.png" height={38} />
        {/* If there are notifications, show their number */}
        {
          notification_number !== 0 ?
            <NotificationCircle notification_number={notification_number} />
            : null
        }
      </div>
      {
        show_notifications ?
          <NotificationList notifications={notifications} />
          : null
      }
    </div>
  </div >
}

function NotificationList(props) {
  const { notifications } = props;
  return <div className="position-absolute border rounded p-3 bg-white" style={{
    top: "50px",
    right: "-20px",
    textAlign: "center",
  }}>
    {
      notifications.map(notification =>
        <div>test</div>
      )
    }
  </div>;
}
