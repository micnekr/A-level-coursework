const Button = ReactBootstrap.Button;
const Image = ReactBootstrap.Image;

/** The component displayed at the top of the screen, like a navbar
 */
function Header() {
    const [is_logged_in, set_is_logged_in] = useState(false);
    const [notifications, set_notifications] = useState([]);

    // Check if logged in when the page is loaded and load notification count
    useEffect(() => {
        request("/api/is_logged_in", set_is_logged_in);
        request("/api/get_notifications", set_notifications);
    }, []);

    /** a function to reply to an invitation
    */
    async function reply_to_group_invitation(group_id, was_accepted) {
        const res = await f("/api/reply_to_group_invitation", "POST", {
            group_id, was_accepted
        });

        // if it was not successful, show the error message
        if (res.status >= 400) {
            // Read the error message
            const error = await res.text();
            return console.error(error);
        }

        // refresh the notifications
        request("/api/get_notifications", set_notifications);
    }

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
        const [show_notifications, set_show_notifications] = useState(false);

        function toggle_show_notification() {
            // only show the notifications if there are some available
            if (notifications.length == 0) return;
            set_show_notifications(!show_notifications);
        }

        // Center it
        return <div className="text-center">
            <div className="position-relative d-inline-block">
                <div style={{ cursor: "pointer" }} onClick={toggle_show_notification}>
          // Display the icon
                    <Image src="/img/notification.png" height={38} />
                    {/* If there are notifications, show their number */}
                    {
                        notification_number !== 0 ?
                            <NotificationCircle notification_number={notification_number} />
                            : null
                    }
                </div>
              // only show the notifications list if there are any notifications
                {
                    show_notifications ?
                        <NotificationList notifications={notifications} />
                        : null
                }
            </div>
        </div >;
    }

    /** A react component to display the list of events
    */
    function NotificationList(props) {
        const { notifications } = props;
        const number_of_items = notifications.length;
        return <div className="position-absolute border rounded p-3 bg-white" style={{
            top: "50px",
            right: "-20px",
            textAlign: "center",
            zIndex: 2
        }}>
            {
                notifications.map((notification, i) => {
                    // Get the data out of a notification
                    let { id, name } = notification.Invitation;

                    const notification_element = <div style={{
                        width: "40vw",
                        minWidth: "250px"
                    }}>
                        // Display the entry for each invitation
                        <span>You have been invited to the group "{name}"</span>
                        <div className="container-fluid mt-2">
                          // Present the choice of accepting or rejecting
                            <div className="row justify-content-around">
                                <Button variant="success" className="col-5" onClick={() => reply_to_group_invitation(id, true)}>Accept</Button>
                                <Button variant="danger" className="col-5" onClick={() => reply_to_group_invitation(id, false)}>Reject</Button>
                            </div>

                        </div>
                    </div>;

                    // Determines if this is the last notification
                    // This is used to place separator bars between notificaitons,
                    // but not after the last one
                    const is_last_element = i === number_of_items - 1;

                    return <div key={i}>
                        {notification_element}
                        {/* Show a separating bar */}
                        {is_last_element ? null : <hr />}
                    </div>;
                }
                )
            }
        </div>;
    }
}
