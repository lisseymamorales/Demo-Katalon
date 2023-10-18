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
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

response = WS.sendRequest(findTestObject('TOREI-DB-SYS/POST-200', [('client_id') : GlobalVariable.client_id, ('client_secret') : GlobalVariable.client_secret
            , ('torei-db-sys-name') : GlobalVariable.toreidbsysname, ('Age') : 25, ('Experience') : 1, ('Income') : '100'
            , ('ZIPCode') : '33130', ('Family') : '1', ('CCAvg') : 0, ('Education') : 'Superior', ('Mortgage') : '500', ('PersonalLoan') : true
            , ('SecuritiesAccount') : false, ('CDAccount') : true, ('cOnline') : true, ('CreditCard') : '378282246310000'
            , ('BasicToken') : "$GlobalVariable.username:$GlobalVariable.password".bytes.encodeBase64().toString(), ('path') : GlobalVariable.path_users
            , ('b') : GlobalVariable.B, ('contentT') : GlobalVariable.contentType]))

WS.verifyResponseStatusCode(response, 200)

if (response.getResponseText().contains('created and added to database')) {
    println('pass')
} else {
    keywordUtil.markFailed()
}

