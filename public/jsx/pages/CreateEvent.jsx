const Button = ReactBootstrap.Button;
const Form = ReactBootstrap.Form;

const useState = React.useState;
const useEffect = React.useEffect;

function CreateEvent() {
  // Remove the seconds and milliseconds, as we are only concerned about dates and hh:mm
  const time_now = dayjs().second(0).millisecond(0);

  /// Converts a dayjs time to a time string used for datetime-local
  function time_to_time_string(time) {
    return time.format("YYYY-MM-DDTHH:mm");
  }

  /// Converts a time string used for datetime-local to a dayjs time
  function time_string_to_time(time) {
    return dayjs(time, "YYYY-MM-DDTHH:mm");
  }

  // Have variables to keep track of the state
  const [title, set_title] = useState("");
  const [recurrence, set_recurrence] = useState("weekly");
  const [visibility, set_visibility] = useState("private");
  const [start_time, set_start_time] = useState(time_now);
  const [end_time, set_end_time] = useState(time_now);

  // em is short for "error message"
  const [title_em, set_title_em] = useState("");
  const [time_em, set_time_em] = useState("");
  const [overall_em, set_overall_em] = useState("");

  // Remove the error messages when typing
  useEffect(() => {
    set_title_em("");
  }, [title]);
  useEffect(() => {
    set_overall_em("");
  }, [title, recurrence, visibility, start_time, end_time])

  return <PageContainerBox title="Create an event">
    <Form>
      <Form.Group className="mb-3" controlId="formBasicTitle">
        <Form.Label>Title</Form.Label>
        <Form.Control type="text" placeholder="Title" value={title} onChange={e => {
          const new_title = e.target.value;
          // Make sure that the length is acceptable
          if (new_title.length > 100) return set_title_em("The title should not be longer than 100 symbols");

          set_title(e.target.value)
        }} />
        <ErrorMessage em={title_em} />
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicActivityRecurrence">
        <Form.Label>How often does this activity happen?</Form.Label>
        <Form.Select value={recurrence} onChange={e => {
          const new_recurrence = e.target.value;
          set_recurrence(new_recurrence);
        }}>
          <option value="weekly">Weekly</option>
          <option value="once">Once</option>
        </Form.Select>
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicStartTime">
        <Form.Label>When does the activity start?</Form.Label>
        <input type="datetime-local"
          className="form-control"
          value={time_to_time_string(start_time)}
          onChange={e => {
            const new_start_time = time_string_to_time(e.target.value);
            // Do not allow end times that happen before the beginning times
            if (end_time.valueOf() < new_start_time.valueOf()) set_time_em("The activity can not end before it has begun");
            // Otherwise, reset the error message
            else set_time_em("");
            set_start_time(new_start_time);
          }}
          onKeyDown={(e) => { e.key === "Enter" && e.preventDefault() }} // Prevent submitting the form by accident
        />
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicEndTime">
        <Form.Label>When does the activity end?</Form.Label>
        <input type="datetime-local"
          className="form-control"
          value={time_to_time_string(end_time)}
          onChange={e => {
            const new_end_time = time_string_to_time(e.target.value);
            // Do not allow end times that happen before the beginning times
            if (new_end_time.valueOf() < start_time.valueOf()) set_time_em("The activity can not end before it has begun");
            // Otherwise, reset the error message
            else set_time_em("");
            set_end_time(new_end_time);
          }}
          onKeyDown={(e) => { e.key === "Enter" && e.preventDefault() }} // Prevent submitting the form by accident
        />
        <ErrorMessage em={time_em} />
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicVisibility">
        <Form.Label>Who can see this activity?</Form.Label>
        <Form.Select value={visibility} onChange={e => {
          const new_visibility = e.target.value;
          set_visibility(new_visibility);
        }}>
          <option value="private">Participants</option>
          <option value="public">Everyone</option>
        </Form.Select>
      </Form.Group>

      <ErrorMessage em={overall_em} />
      <Button variant="primary" onClick={submit}>
        Done
      </Button>
    </Form>
  </PageContainerBox>;

  async function submit() {
    // Check if the error messages are shown and avoid proceeding if they are
    if (title_em !== "" || time_em !== "") return;

    // Check that the title is not empty
    if (title === "") return set_title_em("The title can not be empty");
  }
}
