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

WebUI.openBrowser('')
int startTimeOB = System.currentTimeMillis() /*inicio conteo antes de ingresar a la página*/
WebUI.navigateToUrl('https://demo-store.katalon.com/signin')
int endTimeOB = System.currentTimeMillis() /*fin conteo luego de ingresar a la página*/
int RunTimeOB = ((endTimeOB - startTimeOB)/1000)
println('RuntimeOB seconds: ' + RunTimeOB)

if  (RunTimeOB <= 50) {
	WebUI.setText(findTestObject('Object Repository/Page_Zack Market/input_Email_email'), 'unicidad@email.com')
	WebUI.setEncryptedText(findTestObject('Object Repository/Page_Zack Market/input_Password_password'), 'QJblfja5Cso=')
	int startTimeLog = System.currentTimeMillis() /*inicio de conteo luego de ingresar credenciales y hacer click en login*/
		WebUI.click(findTestObject('Page_Zack Market/Botn Sign in'))
	Thread.sleep(10000) /*delay para esperar por carga de interfaz privada*/
	String url = WebUI.getUrl()
	WebUI.verifyMatch(url, 'https://demo-store.katalon.com/dashboard', true)
	println(url)
	WebUI.verifyElementPresent(findTestObject('Page_Zack Market/button_user_logout'), 0)
	int endTimeLog = System.currentTimeMillis() /*fin de conteo al ingresar a interfaz privada*/
	int RunTimeLog = ((endTimeLog - startTimeLog)/1000)
	println('RuntimeLog seconds: ' + RunTimeLog)
	
		if  (RunTimeLog <= 50){
			WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Zack Market/Page_Zack Market/div_DEPARTMENT'), 0)
			
		    WebUI.verifyElementText(findTestObject('Object Repository/Page_Zack Market/Page_Zack Market/label_Men'), 'Men')
					
			int startTimeFilter = System.currentTimeMillis()
			WebUI.click(findTestObject('Object Repository/Page_Zack Market/Page_Zack Market/div_DEPARTMENT_checkbox_design__sArRO'))
			WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Zack Market/Page_Zack Market/div_Perl Knit Swear79.99 CADOrange'), 
    0)
			int endTimeFilter = System.currentTimeMillis()
			int RunTimeFilter = ((endTimeFilter - startTimeFilter)/1000)
			println('RuntimeFilter seconds: ' + RunTimeFilter)
			
			if  (RunTimeFilter <= 50) {
				println "passed"
			}
			else {
				keywordUtil.markFailed()
			}
		}
		else {
		keywordUtil.markFailed()		
		}	
}else {
	keywordUtil.markFailed()
	}




