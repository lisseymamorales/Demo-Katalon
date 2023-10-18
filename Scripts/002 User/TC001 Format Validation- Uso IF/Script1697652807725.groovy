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

response = WS.sendRequestAndVerify(findTestObject('TOREI-DB-SYS/POST-400', [('client_id') : GlobalVariable.client_id, ('client_secret') : GlobalVariable.client_secret
            , ('BasicToken') : '', ('B') : GlobalVariable.B, ('torei-db-sys-name') : GlobalVariable.toreidbsysname, ('path') : GlobalVariable.path_users
            , ('Age') : Age, ('Experience') : Experience, ('Income') : Income, ('ZIPCode') : ZIPCode, ('Family') : Family
            , ('CCAvg') : CCAvg, ('Education') : Education, ('Mortgage') : Mortgage, ('PersonalLoan') : PersonalLoan, ('SecuritiesAccount') : SecuritiesAccount
            , ('CDAccount') : CDAccount, ('cOnline') : cOnline, ('CreditCard') : CreditCard]))

String errorMessage = WS.getElementPropertyValue(response, 'errorMessage')

println(errorMessage)

if ((((((((((((errorMessage.contains('Age') || errorMessage.contains('Experience')) || errorMessage.contains('Income')) || 
errorMessage.contains('ZIPCode')) || errorMessage.contains('Family')) || errorMessage.contains('CCAvg')) || errorMessage.contains(
    'Education')) || errorMessage.contains('Mortgage')) || errorMessage.contains('PersonalLoan')) || errorMessage.contains(
    'SecuritiesAccount')) || errorMessage.contains('CDAccount')) || errorMessage.contains('cOnline')) || errorMessage.contains(
    'CreditCard')) {
    if ((((((((((((((errorMessage.contains('expected type: Integer, found: Null') || errorMessage.contains('expected type: Integer, found: Double')) || 
    errorMessage.contains('expected type: Integer, found: String')) || errorMessage.contains('expected type: Number, found: Null')) || 
    errorMessage.contains('expected type: Number, found: String')) || errorMessage.contains('expected type: String, found: Null')) || 
    errorMessage.contains('expected type: Boolean, found: Null')) || errorMessage.contains('expected type: Boolean, found: String')) || 
    errorMessage.contains('expected type: Boolean, found: Integer')) || errorMessage.contains('is not greater or equal to')) || 
    errorMessage.contains('is not less or equal to')) || errorMessage.contains('does not match pattern')) || errorMessage.contains(
        'expected minLength')) || errorMessage.contains('expected maxLength')) || errorMessage.contains('is not a valid enum value')) {
        println('pass')
    }
} else {
    keywordUtil.markFailed()
}

String code = WS.getResponseStatusCode(response)

println(code)

WS.verifyEqual(code, 400)

