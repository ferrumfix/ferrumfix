<?xml version="1.0" encoding="UTF-8"?>

<!-- 

Converts 2010 edition file to 2009 compatible file

Revisions
	01-Mar-2010		Phil Oliver
-->

<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
	<xsl:output method="xml" version="1.0" encoding="UTF-8" indent="yes"/>
	
	<xsl:include href="Translate_support.xsl" />

	<xsl:template match="comment()" >
		<xsl:copy/>
	</xsl:template>

	<xsl:template match="processing-instruction()">
	  <xsl:copy/>
	</xsl:template>
	
	<xsl:variable name="sectionTrans">
		<trans a="Session" b="Session" />
		<trans a="PreTrade" b="Pre Trade" />
		<trans a="Trade" b="Trade" />
		<trans a="PostTrade" b="Post Trade" />
		<trans a="Infrastructure" b="Infrastructure" />
		<trans a="Other" b="Other" />										
	</xsl:variable>

	<xsl:template match="Message">
		<xsl:variable name="sectionID" select="SectionID" />
		
		<MsgType>
			<xsl:call-template name="translateEntityLevelAttributes2010to2009"><xsl:with-param name="cur" select="." /></xsl:call-template>			

			<xsl:copy-of select="MsgType" />
			<MessageName><xsl:value-of select="Name" /></MessageName>
			<ComponentType>Message</ComponentType>
			<Category><xsl:value-of select="CategoryID" /></Category>
			<MsgID><xsl:value-of select="ComponentID" /></MsgID>
			<Section><xsl:value-of select="$sectionTrans/trans[@a=$sectionID]/@b" /></Section>	
			<xsl:copy-of select="AbbrName" />					
			<xsl:copy-of select="NotReqXML" />		
			<xsl:copy-of select="Description" />					
		</MsgType>
	</xsl:template>		

	<xsl:template match="Messages">
		<dataroot copyright="Copyright (c) FIX Protocol Ltd. All Rights Reserved." edition="2009" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"  xsi:noNamespaceSchemaLocation="../../schema/MsgType.xsd">
			<xsl:copy-of select="@version" />
			<xsl:copy-of select="@generated" />
			<xsl:if test="@latestEP"><xsl:attribute name="latestEP">EP<xsl:value-of select="@latestEP" /></xsl:attribute></xsl:if>
			<xsl:apply-templates />				
		</dataroot>
	</xsl:template>	
	
	<xsl:template match="/">
		<xsl:apply-templates />
	</xsl:template>
</xsl:stylesheet>
