// Imports
const Button = ReactBootstrap.Button;
const Pagination = ReactBootstrap.Pagination;

const useState = React.useState;
const useEffect = React.useEffect;

function Calendar() {
    const [start_of_week_date, set_start_of_week_date] = useState(
        dayjs(new Date()).startOf("week").add(1, "day")
    ); // Their week starts on a Sunday, so we add 1 to make it a Monday
    const start_of_next_week_date = start_of_week_date.add(1, "w");

    const [events, set_events] = useState([]);
    // em is short for "error message"
    const [overall_em, set_overall_em] = useState("");

    // Load the events when the page is loaded, once
    useEffect(() => {
        async function get_data() {
            const res = await f("/api/get_events", "GET");
            // If there was an error, display it
            if (res.status >= 400) {
                // Read the error message
                const error = await res.text();
                return set_overall_em(error);
            }
            // Parse the response
            const events = (await res.json()).events;
            set_events(events);
        }
        get_data();
    }, [])

    return <PageContainerBoxLarge title="Your Calendar">
            // An error message that is only shown if there is a request error
        <ErrorMessage em={overall_em} />
        // Display different weeks on different pages
        <Pagination className="container">
            // Navigation buttons
            <div className="row justify-content-center w-100 gx-0">
                <Pagination.Prev className="col-auto" onClick={() => {
                    // Go to the previous week
                    set_start_of_week_date(start_of_week_date.subtract(1, "w"));
                }} />

                // The current page; displays the week start and end
                <Pagination.Item className="col-auto">{start_of_week_date.format("DD/MM/YYYY")} - {start_of_week_date.add(6, "d").format("DD/MM/YYYY")}</Pagination.Item>

                <Pagination.Next className="col-auto" onClick={() => {
                    // Go to the next week
                    set_start_of_week_date(start_of_week_date.add(1, "w"));
                }} />
            </div>
        </Pagination>
        // The actual timetable
        <Timetable events={events} start_of_week_date={start_of_week_date} start_of_next_week_date={start_of_next_week_date} />
             // "New event" button; reidrect on click
        <Button variant="primary" onClick={() => {
            // Redirect to the event creation page
            window.location.href = "/create_event";
        }}
            className="mt-3 mx-auto d-block w-60"
        >
            New event
        </Button>
    </PageContainerBoxLarge>;
}
