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
import java.time.OffsetDateTime as OffsetDateTime
import java.time.ZoneOffset as ZoneOffset
import java.time.format.DateTimeFormatter as DateTimeFormatter
import java.time.format.DateTimeFormatterBuilder as DateTimeFormatterBuilder
import java.util.Locale as Locale

Date todaysDate = new Date();
println todaysDate
def formattedDate = todaysDate.format("dd-MMM-yyyy HH:MM");


println formattedDate

println todaysDate

for (i=0; i<5;i++) {
	String myEmailAddress = "thisismyaddress" + System.nanoTime() + "@Gmail.com";
	System.out.println(myEmailAddress);
}

for (i=0; i<5;i++) {
	def todaysDate1 = new Date(100);
	println todaysDate1
	println todaysDate1.getTime()
}

