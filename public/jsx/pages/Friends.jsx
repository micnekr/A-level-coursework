const Form = ReactBootstrap.Form;

function Friends() {

  const [overall_em, set_overall_em] = useState("");
  const [friendship_groups, set_friendship_groups] = useState([]);
  const [friends, set_friends] = useState([]);
  const [new_friend_username, set_new_friend_username] = useState("");

  // Fetch the data from the server
  useEffect(() => {
    set_overall_em("");

    refresh_groups_list();

    refresh_friends_list();
  }, [])

  /** A function that refreshes the list of groups and their participants
  */
  function refresh_groups_list() {
    request("/api/get_owned_groups_with_participants", (data) => {
      // reshape the data
      const friendship_groups = data.groups.map(el => { return { name: el.name, id: el.id, participants: [] } });
      //
      // Add the users to their corresponding groups
      for (let user of data.participants) {
        const corresponding_group = friendship_groups.find(el => el.id == user.group_id);
        corresponding_group.participants.push(user);
      }

      set_friendship_groups(friendship_groups);
    });
  }

  /** A function that refreshes the list of friends
  */
  function refresh_friends_list() {
    request("/api/get_friends", set_friends);
  }

  /** A function to add a friend by username
  */
  async function add_friend() {
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

    refresh_friends_list();
  }

  /** A function to create a new group
  */
  async function create_group() {
    const res = await f("/api/create_group", "POST", {
      name: "New Group",
    });

    // if it was not successful, show the error message
    if (res.status >= 400) {
      // Read the error message
      const error = await res.text();
      return set_overall_em(error);
    }

    refresh_groups_list();
  }

  return <PageContainerBox>
    <h2 className="mx-auto text-center">Friendship groups</h2>
    {/* Display the different available groups */}
    {friendship_groups.map((group, i) => <FriendshipGroup group={group} friends={friends} key={i} />
    )}

    {/* A button to add a new group */}
    <Button className="d-block mx-auto" variant="outline-primary" style={{ cursor: "pointer" }} onClick={create_group}>
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
          <Button className="h-100 w-100" variant="primary" type="submit" onClick={add_friend}>
            Add
          </Button>
        </div>
      </div>
    </div>
    <ErrorMessage em={overall_em} />
  </PageContainerBox >;

  /** An element of the page that shows the name of a friendship group and users, letting the users be added or deleted
  */
  function FriendshipGroup(props) {
    const { group, friends } = props;
    const { name, participants } = group;

    // Is it showing the form to add a new friend to the list?
    const [is_showing_popup, set_is_showing_popup] = useState(false);
    // What is the name of the friend to be added to the list?
    // -1 means an invalid id
    const [new_friend_id, set_new_friend_id] = useState(-1);

    /** A function to invite a user to a group
    */
    async function invite_to_group(new_friend_id, group_id) {
      const res = await f("/api/invite_to_group", "POST", {
        user_id: new_friend_id,
        group_id
      });

      // if it was not successful, show the error message
      if (res.status >= 400) {
        // Read the error message
        const error = await res.text();
        return set_overall_em(error);
      }

      refresh_groups_list();
    }

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
        {participants.map((user, i) => <div className="col-6 col-md-2" key={i}>
          <div className="text-center border rounded">
            <div className="align-middle d-inline-block text-truncate" style={{ maxWidth: "70%" }}>{user.username}</div>
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
                <Form.Select value={new_friend_id} onChange={e => set_new_friend_id(e.target.value)}>
                  {/* Only show the default option if no valid person was selected */}
                  {new_friend_id === -1 ?
                    <option value={-1}>Please select a friend</option> : null
                  }
                  {friends
                    // make sure not to suggest friends who are already in the group
                    .filter(potential_friend => participants.every(displayed_friend => potential_friend.id != displayed_friend.user_id))
                    .map((friend, i) =>
                      <option value={friend.id} key={i}>{friend.username}</option>
                    )}
                </Form.Select>
              </div>
              <div className="col-sm-4 col-12">
                <Button className="h-100 w-100" variant="primary" type="submit" onClick={() => {
                  // Close the popup and add the user to the group
                  set_is_showing_popup(false);
                  const user_id = parseInt(new_friend_id);
                  // if the user was not selected, do not do anything
                  if (user_id == -1) return;

                  invite_to_group(user_id, group.id);
                }}>
                  Add
                </Button>
              </div>
            </div>
          </div>}
      </div>
    </div>
  }
}
