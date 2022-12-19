const Form = ReactBootstrap.Form;

function Friends() {

  const [overall_em, set_overall_em] = useState("");
  const [friendship_groups, set_friendship_groups] = useState([]);
  const [friends, set_friends] = useState([]);
  const [new_friend_username, set_new_friend_username] = useState("");

  // Fetch the data from the server
  useEffect(() => {

    const friendship_group_data = [{
      name: "Golf",
      participants: ["John", "some very long name"]
    }, {
      name: "Golf2",
      participants: ["George"]
    }];

    set_overall_em("Some error that might occur");

    set_friendship_groups(friendship_group_data);

    request("/api/get_friends", set_friends);
  }, [])

  /** A function to add a friend by username
  */
  async function addFriend() {
    // Remove the error so that the user can see the new error or the fact that there is no error
    set_overall_em("");
    const res = await f("/api/add_friend", "POST", {
      username: new_friend_username
    });

    // if it was not successful, show the error message
    if (res.status >= 400) {
      // Read the error message
      const error = await res.text();
      return set_overall_em(error);
    }
  }

  return <PageContainerBox>
    <h2 className="mx-auto text-center">Friendship groups</h2>
    {/* Display the different available groups */}
    {friendship_groups.map((group, i) => <FriendshipGroup group={group} friends={friends} key={i} />
    )}

    {/* A button to add a new group */}
    <Button className="d-block mx-auto" variant="outline-primary" style={{ cursor: "pointer" }}>
      <i className="fas fa-plus" /> New group
    </Button>

    {/* A button to add a new friend */}
    <h2 className="mx-auto text-center mt-3">Add Friend</h2>
    <div className="container-fluid p-0">
      <div className="row justify-content-between gy-3">
        <div className="col-sm-8 col-12">
          <input className="border rounded px-2 h-100 w-100" type="text" placeholder="Friend username" value={new_friend_username} onChange={e => set_new_friend_username(e.target.value)} />
        </div>
        <div className="col-sm-4 col-12">
          <Button className="h-100 w-100" variant="primary" type="submit" onClick={addFriend}>
            Add
          </Button>
        </div>
      </div>
    </div>
    <ErrorMessage em={overall_em} />
  </PageContainerBox >;
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

  return <div className="container border rounded py-2 mb-2" >
    <div className="row">
      <div className="col">
        <span className="ps-3">
          {name}
        </span>
        <i className="far fa-edit ps-2" data-fa-transform="grow-10 up-1" style={{ cursor: "pointer" }} />
      </div>
    </div>
    {/* Display the different participants */}
    <div className="row justify-content-start g-2">
      {participants.map((name, i) => <div className="col-6 col-md-2" key={i}>
        <div className="text-center border rounded">
          <div className="align-middle d-inline-block text-truncate" style={{ maxWidth: "70%" }}>{name}</div>
          <i className="fas fa-times ps-1" style={{ cursor: "pointer" }} data-fa-transform="grow-5 down-4" />
        </div>
      </div>)}
      {/* Toggle the popup on click */}
      <div className="col-1" onClick={() => set_is_showing_popup(!is_showing_popup)} >
        <i className="fas fa-plus ps-1" style={{ cursor: "pointer" }} data-fa-transform="grow-5 down-4" />
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
