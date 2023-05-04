/** A component that displays an event within a timetable
 */
function TimetableEvent(props) {
    const { container_ref, event } = props;
    const { start_time, end_time, duration, title } = event;

    const beginning_of_day = start_time.startOf("day");
    const end_of_day = start_time.endOf("day");
    const seconds_in_time_period = end_of_day.unix() - beginning_of_day.unix();

    // The gap between the event bubble and the containing column, horizontally
    const horizontal_gap = 4;

    // Keep track of the height and width as it gets updated after the ref gets set
    const [container_height, set_container_height] = useState(0);
    const [container_width, set_container_width] = useState(0);

    // Reset the height on window resize
    useWindowResize(() => {
        set_container_height(container_ref.current.clientHeight);
        set_container_width(container_ref.current.clientWidth);
    })

    /** A function to convert a date object into a time string
    */
    function display_time(date) {
        return date.format("HH:mm");
    }

    /** Linearly interpolates n that is between min_n and max_n into the range between min_out and max_out
    */
    function interpolate(n, min_n, max_n, min_out, max_out) {
        return min_out + (n - min_n) / (max_n - min_n) * max_out;
    }

    // Set the borders of the event
    return <div style={{
        top: `${interpolate(start_time.unix(), beginning_of_day.unix(), end_of_day.unix(), 0, container_height)}px`,// interpolate between the beginning and end of day
        height: `${interpolate(duration, 0, seconds_in_time_period, 0, container_height)}px`,// interpolate between the beginning and end of day
        width: `${container_width - horizontal_gap}px`, // leave a gap
        left: `${horizontal_gap / 2}px`, // Center

        // Change the colour back to normal
        backgroundColor: "white"
    }}
        className="position-absolute border rounded"
    >


           // Display the event data itself
        <div className="container h-100 gx-0">
            <div className="row h-100">
                <div className="col align-self-center">
                    <em className="fs-6 d-block text-truncate text-center">{title}</em>
                    <div className="fs-6 text-center text-muted">{display_time(start_time)}-{display_time(end_time)}</div>
                </div>
            </div>

        </div>
    </div>
}
