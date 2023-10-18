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

WebUI.openBrowser('https://petstore.octoperf.com/')

WebUI.click(findTestObject('Object Repository/Page_JPetStore Demo/a_Enter the Store'))

WebUI.click(findTestObject('Object Repository/Page_JPetStore Demo/img'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_JPetStore Demo/h2_Fish'), 'Fish')

println fish = WebUI.getUrl()

if (fish.contains('FISH')) {

	WebUI.click(findTestObject('Object Repository/Page_JPetStore Demo/a_Return to Main Menu'))
	WebUI.click(findTestObject('Object Repository/Page_JPetStore Demo/img_1'))
	WebUI.verifyElementText(findTestObject('Object Repository/Page_JPetStore Demo/h2_Dogs'), 'Dogs')
	println dog = WebUI.getUrl()
	
	if (dog.contains('DOGS')) {
		
		WebUI.click(findTestObject('Object Repository/Page_JPetStore Demo/a_Return to Main Menu'))
		
		WebUI.click(findTestObject('Object Repository/Page_JPetStore Demo/img_1_2'))
		
		WebUI.verifyElementText(findTestObject('Object Repository/Page_JPetStore Demo/h2_Cats'), 'Cats')
		
		println cat = WebUI.getUrl()
		
		if (cat.contains('CATS')) {
			
			WebUI.click(findTestObject('Object Repository/Page_JPetStore Demo/a_Return to Main Menu'))
			
			WebUI.closeBrowser()
		}
	}
	
	
}











