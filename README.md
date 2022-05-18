# notch-farm
Backend server for managing and tracking who won internet arguments


#### API Routes
- [x] `POST /arguments` create an argument
- [x] `GET /arguments?group_id={group_id}` get all arguments for the group
- [x] `GET /arguments/{argument_id}` - get argument with given id
- [x] `PUT /arguments/{argument_id}/notch-taker` - update argument with given id

#### Discord Commands
- [ ] `/notch help` - help text
- [ ] `/notch argument @otherUser "{description}"` - starts or proposes an internet argument with 
another user with the given description
- [ ] `/notch arguments` - show all open arguments
- [ ] `/notch take_your_notch {argument_id}` - allows the argument to be taken by other member
- [ ] `/notch leaderboard` - shows ordered list of notch counts in server