const Form = ReactBootstrap.Form;

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
  const [recurrence, set_recurrence] = useState("Weekly");
  const [visibility, set_visibility] = useState("Private");
  const [group_id, set_group_id] = useState(-1); // represents an invalid group id
  const [start_time, set_start_time] = useState(time_now);
  const [end_time, set_end_time] = useState(time_now);

  // em is short for "error message"
  const [title_em, set_title_em] = useState("");
  const [time_em, set_time_em] = useState("");
  const [overall_em, set_overall_em] = useState("");

  const [groups, set_groups] = useState([]);

  // Load the groups once ready
  useEffect(() => {
    request("/api/get_owned_groups_with_participants", (data) => {
      const groups = data.groups.map(el => ({ name: el.name, id: el.id }));
      set_groups(groups);
      // Set the group id to the first group id
      set_group_id(groups[0].id);
    });
  }, []);


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
          <option value="Weekly">Weekly</option>
          <option value="Once">Once</option>
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
          <option value="Private">Participants</option>
          <option value="Public">Everyone</option>
        </Form.Select>
      </Form.Group>

      <Form.Group className="mb-3" controlId="formBasicGroup">
        <Form.Label>What group is this activity for?</Form.Label>
        <Form.Select value={group_id} onChange={e => {
          // convert to an integer
          set_group_id(parseInt(e.target.value));
        }}>
          {/* An option for each group */}
          {
            groups.map((group, i) =>
              <option value={group.id} key={i}>{group.name}</option>
            )
          }
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
    // Check that the times are not empty, either
    if (isNaN(start_time.unix()) || isNaN(end_time.unix())) return set_time_em("Please enter valid times");

    // Make the actual request
    const res = await f("/api/create_event", "POST", {
      title,
      visibility,
      recurrence,
      group_id,
      start_time: start_time.unix(),
      duration: end_time.unix() - start_time.unix(),
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
