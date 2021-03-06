use hyper::{Body, Method, Request, Client};

async fn post_data(req_body:String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {


    let req = Request::builder()
    .method(Method::POST)
    .uri("http://173.249.46.156:8080/abunzi_mis_service/api/minCases/addCase")
    .header("content-type", "application/json")
    .header("Authorization", "Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJ1bXVtYWoiLCJleHAiOjE2MjU3OTkwNTksImlhdCI6MTYyNTc4MTA1OX0.V_ZGohPVN9cedowOGaoYRwNMIZpJwVZ5tHw6v0QyliD2bfVn_VQSd-2N1EZRshao_q4szo6RUWj1FKy8WzfuXQ")
    .body(Body::from(req_body))?;

    // We'll send it in a second...

    let client = Client::new();

    
    // POST it...
    let resp = client.request(req).await?;

    println!("{:?} {:?}", resp.status(),resp.headers());

    Ok(())
}

#[tokio::main]
#[allow(unused_must_use)]
async fn main()  {
    let req_body = r#"
    {
        "assignedToId":1,
        "birthDate": "2001-06-02",
        "caseDate": "2021-06-02 14:03:12",
        "caseDescription": "descriptions",
        "caseGroupe": "CIVIL",
        "caseStatus": "DRAFT",
        "caseType": "NEW_CASE",
        "categoryId": 6,
        "docType": "NID",
        "fatherNames": "f names",
        "firstName": "first name",
        "isDesabled": "YES",
        "lastName": "last name",
        "legalAidProvided": "LEGAL_ADVICE",
        "locationId": 21911,
        "maritalStatus": "MARRIED",
        "medium": "WEB",
        "motherNames": "m names",
        "nationalId": "1234567890123456",
        "nidNumber": "1234567890123456",
        "phoneNumber": "0987654321",
        "placeOfBirth": "nyamasheke",
        "placeOfResidence": "nyamasheke",
        "publicFundToBeRecovered": 1000,
        "reporter": "MEDIATOR",
        "residenceLocationId":  21911,
        "respondent": {
            "birthDate": "2021-06-02",
            "docType": "NID",
            "fatherNames": "f names",
            "firstName": "first name",
            "isDesabled": "YES",
            "lastName": "last name",
            "maritalStatus": "MARRIED",
            "motherNames": "m names",
            "nidNumber": "1234567890123456",
            "phoneNumber": "0987654321",
            "placeOfBirth": "nyamasheke",
            "placeOfResidence": "nyamasheke",
            "residenceLocationId": 34862,
            "sex": "MALE"
        },
        "sex": "MALE",
        "summaryOfFacts": "some summary of Facts",
        "userId": 6
    }
    "#;

    for x in 1..100000{
        println!("{} time : ", x);
        post_data(String::from(req_body)).await;
    }

    // Ok(())
}

