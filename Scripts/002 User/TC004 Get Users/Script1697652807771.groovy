import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager


response = WS.sendRequestAndVerify(findTestObject('TOREI-DB-SYS/GET-user', [('client_id') : GlobalVariable.client_id, ('client_secret') : GlobalVariable.client_secret
            , ('torei-db-sys-name') : GlobalVariable.toreidbsysname, ('BasicToken') : "$GlobalVariable.username:$GlobalVariable.password".bytes.encodeBase64().toString()
            , ('B') : GlobalVariable.B, ('path') : GlobalVariable.path_users]))

def jsonSlurper = new JsonSlurper()

def parsedJson_count = jsonSlurper.parseText(response.getResponseText())

def expectedSize = parsedJson_count.size()

String value

println('Total: ' + expectedSize)

println parsedJson_count

String code = WS.getResponseStatusCode(response)

println(code)

WS.verifyEqual(code, 200)

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

String jsonPass = '''

{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "array",
  "items": [
    {
      "type": "object",
      "properties": {
        "Income": {
          "type": "integer"
        },
        "Education": {
          "type": "string"
        },
        "cOnline": {
          "type": "integer"
        },
        "Experience": {
          "type": "integer"
        },
        "SecuritiesAccount": {
          "type": "integer"
        },
        "CreditCard": {
          "type": "string"
        },
        "PersonalLoan": {
          "type": "integer"
        },
        "ZIPCode": {
          "type": "string"
        },
        "CCAvg": {
          "type": "integer"
        },
        "Mortgage": {
          "type": "integer"
        },
        "CDAccount": {
          "type": "integer"
        },
        "ID": {
          "type": "integer"
        },
        "Family": {
          "type": "integer"
        },
        "AGE": {
          "type": "integer"
        }
      },
      "required": [
        "Income",
        "Education",
        "cOnline",
        "Experience",
        "SecuritiesAccount",
        "CreditCard",
        "PersonalLoan",
        "ZIPCode",
        "CCAvg",
        "Mortgage",
        "CDAccount",
        "ID",
        "Family",
        "AGE"
      ]
    },
    {
      "type": "object",
      "properties": {
        "Income": {
          "type": "integer"
        },
        "Education": {
          "type": "string"
        },
        "cOnline": {
          "type": "integer"
        },
        "Experience": {
          "type": "integer"
        },
        "SecuritiesAccount": {
          "type": "integer"
        },
        "CreditCard": {
          "type": "string"
        },
        "PersonalLoan": {
          "type": "integer"
        },
        "ZIPCode": {
          "type": "string"
        },
        "CCAvg": {
          "type": "integer"
        },
        "Mortgage": {
          "type": "integer"
        },
        "CDAccount": {
          "type": "integer"
        },
        "ID": {
          "type": "integer"
        },
        "Family": {
          "type": "integer"
        },
        "AGE": {
          "type": "integer"
        }
      },
      "required": [
        "Income",
        "Education",
        "cOnline",
        "Experience",
        "SecuritiesAccount",
        "CreditCard",
        "PersonalLoan",
        "ZIPCode",
        "CCAvg",
        "Mortgage",
        "CDAccount",
        "ID",
        "Family",
        "AGE"
      ]
    }
  ]
}

'''


WS.validateJsonAgainstSchema(response, jsonPass);