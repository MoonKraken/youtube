#
#
# main() will be run when you invoke this action
#
# @param Cloud Functions actions accept a single parameter, which must be a JSON object.
#
# @return The output of this action, which must be a JSON object.
#
#
from cloudant.client import Cloudant
from cloudant.error import CloudantException
from cloudant.result import Result, ResultByKey
import sys
import json
import time
import timeit

def main(dict):
    cloudant_creds = dict.get('__bx_creds').get('cloudantnosqldb')
    ACCOUNT_NAME = cloudant_creds.get('host').split('.')[0]
    API_KEY = cloudant_creds.get('apikey')
    
    print(ACCOUNT_NAME)
    print(API_KEY)

    client = Cloudant.iam(ACCOUNT_NAME, API_KEY, connect=True)
    client.connect()
    
    databaseName = "databasedemo3"
    myDatabaseDemo = client.create_database(databaseName)
    
    if myDatabaseDemo.exists():
        print("'{0}' successfully created.\n".format(databaseName))
    
    sampleData = [
         [1, "one", "boiling", 100],
         [2, "two", "hot", 40],
         [3, "three", "hot", 75],
         [4, "four", "hot", 97],
         [5, "five", "warm", 20],
         [6, "six", "cold", 10],
         [7, "seven", "freezing", 0],
         [8, "eight", "freezing", -5]
    ]
    
    for document in sampleData:
        # Retrieve the fields in each row.
        number = document[0]
        name = document[1]
        description = document[2]
        temperature = document[3]
        jsonDocument = {
            "numberField": number,
            "nameField": name,
            "descriptionField": description,
            "temperatureField": temperature
        }
    
        start = timeit.default_timer()
        newDocument = myDatabaseDemo.create_document(jsonDocument)
        end = timeit.default_timer()
    
        if newDocument.exists():
            print("Document '{0}' successfully created.".format(number))
            print("Write time: {0}".format(end - start))
    
    start = timeit.default_timer()
    result_collection = Result (myDatabaseDemo.all_docs, include_docs=True)
    end = timeit.default_timer()
    print("Retrieved minimal document:\n{0}\n".format(result_collection[0]))
    print("Read time: {0}".format(end - start))
    
    try:
        client.delete_database(databaseName)
    except CloudantException:
        print("There was a problem deleting '{0}'.\n".format(databaseName))
    else:
        print("'{0}' successfully deleted.\n".format(databaseName))
    
    client.disconnect()
    return { 'message': 'Hello world' }