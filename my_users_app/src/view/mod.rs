// <h1>Users</h1>
// <table>
//     <tr>
//         <th>FirstName</th>
//         <th>LastName</th>
//         <th>Age</th>
//         <th>Email</th>
//     </tr>
//     <%  User.all.collect do |row| %>
//     <tr>
//         <td><%= row.firstname%></td>
//         <td><%= row.lastname%></td>
//         <td><%= row.age%></td>
//         <td><%= row.email%></td>
//     </tr>
//     <% end %>
// </table>

use yew::prelude::*;

use crate::db_data::UserNoPassword;

pub fn body(_db: Vec<UserNoPassword>) -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}