<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="2.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:fo="http://www.w3.org/1999/XSL/Format" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:fn="http://www.w3.org/2005/xpath-functions">

	<xsl:template name="translateEntityLevelAttributes2009to2010">
		<xsl:param name="cur" />
		
		<xsl:if test="$cur/@Added">
			<xsl:attribute name="added"><xsl:value-of select="$cur/@Added" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@AddedEP">
			<xsl:attribute name="addedEP"><xsl:value-of select="substring($cur/@AddedEP,3)" /></xsl:attribute>
		</xsl:if>						
		<xsl:if test="$cur/@Deprecated">
			<xsl:attribute name="deprecated"><xsl:value-of select="$cur/@Deprecated" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@Removed">
			<xsl:attribute name="removed"><xsl:value-of select="$cur/@Removed" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@RemovedEP">
			<xsl:attribute name="removedEP"><xsl:value-of select="substring($cur/@RemovedEP,3)" /></xsl:attribute>
		</xsl:if>						
		<xsl:if test="$cur/@Updated">
			<xsl:attribute name="updated"><xsl:value-of select="$cur/@Updated" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@UpdatedEP">
			<xsl:attribute name="updatedEP"><xsl:value-of select="substring($cur/@UpdatedEP,3)" /></xsl:attribute>
		</xsl:if>
	</xsl:template>	

	<xsl:template name="translateEntityLevelAttributes2010to2009">
		<xsl:param name="cur" />
		
		<xsl:if test="$cur/@added">
			<xsl:attribute name="Added"><xsl:value-of select="$cur/@added" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@addedEP">
			<xsl:attribute name="AddedEP">EP<xsl:value-of select="$cur/@addedEP" /></xsl:attribute>
		</xsl:if>						
		<xsl:if test="$cur/@deprecated">
			<xsl:attribute name="Deprecated"><xsl:value-of select="$cur/@deprecated" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@removed">
			<xsl:attribute name="Removed"><xsl:value-of select="$cur/@removed" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@removedEP">
			<xsl:attribute name="RemovedEP">EP<xsl:value-of select="$cur/@removedEP" /></xsl:attribute>
		</xsl:if>						
		<xsl:if test="$cur/@updated">
			<xsl:attribute name="Updated"><xsl:value-of select="$cur/@updated" /></xsl:attribute>
		</xsl:if>
		<xsl:if test="$cur/@updatedEP">
			<xsl:attribute name="UpdatedEP">EP<xsl:value-of select="$cur/@updatedEP" /></xsl:attribute>
		</xsl:if>
	</xsl:template>

</xsl:stylesheet>
