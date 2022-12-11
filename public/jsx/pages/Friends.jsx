const Form = ReactBootstrap.Form;

function Friends() {

  const [overall_em, set_overall_em] = useState("");
  const [friendship_groups, set_friendship_groups] = useState([]);
  const [friends, set_friends] = useState([]);

  // Fetch the data from the server
  useEffect(() => {

    const friendship_group_data = [{
      name: "Golf",
      participants: ["John", "some very long name"]
    }, {
      name: "Golf2",
      participants: ["George"]
    }];

    const friends_data = ["John", "some very long name", "George", "Someone else"];

    set_overall_em("Some error that might occur");

    set_friendship_groups(friendship_group_data);
    set_friends(friends_data);

  }, [])

  return <PageContainerBox>
    <h2 className="mx-auto text-center">Friendship groups</h2>
    {/* Display the different available groups */}
    {friendship_groups.map((group, i) => <FriendshipGroup group={group} friends={friends} key={i} />
    )}

    {/* A button to add a new group */}
    <Button className="d-block mx-auto" variant="outline-primary" style={{ cursor: "pointer" }}>
      <img src="/img/plus.png" style={{ height: "1rem", }} /> New group
    </Button>

    {/* A button to add a new friend */}
    <h2 className="mx-auto text-center mt-3">Add Friend</h2>
    <div className="container-fluid p-0">
      <div className="row justify-content-between gy-3">
        <div className="col-sm-8 col-12">
          <input className="border rounded px-2 h-100 w-100" type="text" placeholder="Friend username" />
        </div>
        <div className="col-sm-4 col-12">
          <Button className="h-100 w-100" variant="primary" type="submit">
            Add
          </Button>
        </div>
      </div>
    </div>
    <ErrorMessage em={overall_em} />
  </PageContainerBox>;
}

/** An element of the page that shows the name of a friendship group and users, letting the users be added or deleted
*/
function FriendshipGroup(props) {
  const { group, friends } = props;
  const { name, participants } = group;

  // Is it showing the form to add a new friend to the list?
  const [is_showing_popup, set_is_showing_popup] = useState(false);
  // What is the name of the friend to be added to the list?
  const [new_username, set_new_username] = useState("");

  useEffect(() => console.log(new_username));

  return <div className="container border rounded py-2 mb-2" >
    <div className="row">
      <div className="col">
        <span className="ps-3">
          {name}
        </span>
        <img className="ps-1 d-inline-block" src="/img/edit-icon.png" style={{ height: "1rem", cursor: "pointer" }}></img>
      </div>
    </div>
    {/* Display the different participants */}
    <div className="row justify-content-start g-2">
      {participants.map((name, i) => <div className="col-6 col-md-2" key={i}>
        <div className="text-center border rounded">
          <div className="align-middle d-inline-block text-truncate" style={{ maxWidth: "70%" }}>{name}</div>
          <img className="align-middle ps-1 d-inline-block ms-auto" src="/img/cross.png" style={{ height: "1rem", cursor: "pointer" }}></img>
        </div>
      </div>)}
      <div className="col-1">
        <img src="/img/plus.png" style={{ height: "1rem", cursor: "pointer" }} onClick={() => set_is_showing_popup(!is_showing_popup)} />
      </div>
      {/* Show a popup to add a friend when this button is clicked */}
      {!is_showing_popup ? null :
        <div className="container-fluid px-0 pt-2">
          <div className="row justify-content-between gy-3">
            <div className="col-sm-8 col-12">
              <Form.Select value={new_username} onChange={e => set_new_username(e.target.value)}>
                {/* Only show the default option if no valid person was selected */}
                {new_username === "" ?
                  <option>Please select a person</option> : null
                }
                {friends.map((friend, i) =>
                  <option value={friend} key={i}>{friend}</option>
                )}
              </Form.Select>
            </div>
            <div className="col-sm-4 col-12">
              <Button className="h-100 w-100" variant="primary" type="submit" onClick={() => set_is_showing_popup(false)}>
                Add
              </Button>
            </div>
          </div>
        </div>}
    </div>
  </div>
}
