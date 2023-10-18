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

WebUI.callTestCase(findTestCase('001 DemoStore/07 Login Exitoso/TC032-Login exitoso'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Zack Market/span_Zack Market'))

WebUI.rightClick(findTestObject('Object Repository/Page_Zack Market/li_Logout'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Zack Market/a_Logout'), 0)

WebUI.click(findTestObject('Object Repository/Page_Zack Market/a_Logout'))

WebUI.navigateToUrl('https://demo-store.katalon.com/')

WebUI.click(findTestObject('Object Repository/Page_Zack Market/div_Zack Market_user'))

println WebUI.getUrl()

WebUI.verifyMatch(WebUI.getUrl(), 'https://demo-store.katalon.com/signin', true)

WebUI.back()

println WebUI.getUrl()

WebUI.verifyMatch(WebUI.getUrl(), 'https://demo-store.katalon.com/', true)

