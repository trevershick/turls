
/api/v1/
/* with content negotiation

Primary 
----
* CRUD
  * ✅ delete by id
  * ✅ create url
    * ✅ fail on dupe 
    * ✅ validate incoming values
  * ✅ get by id
  * search by keyword
  * ✅ action = expand /api/v1/keywords/<keyword> returns just the url string
  * update - change keyword
* version
* auth
* persistence

Secondary
----

* track creation date
* action = stats?
